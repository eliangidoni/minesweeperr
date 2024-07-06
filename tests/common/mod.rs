use axum::Router;
use minesweeperrust::{database::Database, handler, service::Service};

const DATABASE_URL: &str = "postgresql://postgres:postgres@db:5432/minesweeper";

pub async fn setup() -> (Router, Database) {
    let db = setup_database().await;
    let service = Service::new(db.clone());
    let handler = handler::Handler::new(service);
    let router = Router::new().merge(handler.router());
    (router, db)
}

pub async fn setup_database() -> Database {
    let (mut client, conn) = tokio_postgres::connect(DATABASE_URL, tokio_postgres::NoTls)
        .await
        .unwrap();
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    Database::run_migrations(&mut client)
        .await
        .expect("error running migrations");
    Database::new(client)
}
