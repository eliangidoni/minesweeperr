use axum::{http::StatusCode, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use minesweeperrust::{database::Database, handler, service::Service};
use std::env;
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let metrics_port = env::var("METRICS_PORT").unwrap_or("8081".to_string());

    let (mut client, conn) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .expect("error creating connection pool");
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    if let Err(e) = Database::run_migrations(&mut client).await {
        eprintln!("error running migrations: {}", e);
    };
    let db = Database::new(client);
    let service = Service::new(db);
    let handler = handler::Handler::new(service);
    let router = handler
        .router()
        .route_layer(axum::middleware::from_fn(track_metrics))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { StatusCode::OK }))
        .merge(router);
    let metrics_app = setup_metrics_app().await;

    tokio::join!(
        async {
            let addr = format!("0.0.0.0:{}", metrics_port);
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("error listening on port");
            tracing::info!("metrics listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, metrics_app)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .expect("error running metrics serve()");
        },
        async {
            let addr = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("error listening on port");
            tracing::info!("api listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .expect("error running serve()")
        }
    );
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn setup_metrics_app() -> Router {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    let recorder_handle = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap();

    let app = Router::new().route(
        "/metrics",
        axum::routing::get(move || std::future::ready(recorder_handle.render())),
    );
    app
}

async fn track_metrics(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    let start = tokio::time::Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<axum::extract::MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];
    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);
    response
}
