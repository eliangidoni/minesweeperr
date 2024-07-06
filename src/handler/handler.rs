use crate::handler::model::map_to_model;
use crate::handler::model::Game;
use crate::service::Error;
use crate::service::ServiceTrait;
use axum::extract;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{async_trait, Json};
use axum::{extract::Path, extract::State, routing::get, Router};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait HandlerTrait {
    async fn new_game(&self, rows: i32, cols: i32, mines: i32) -> Result<Game, Error>;
    async fn get_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn pause_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn resume_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn mark_as_flag(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error>;
    async fn mark_as_question(&self, game_id: uuid::Uuid, point: (i32, i32))
        -> Result<Game, Error>;
    async fn reveal(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error>;
}

#[derive(Debug, Clone)]
pub struct Handler {
    service: Arc<dyn ServiceTrait + Send + Sync>,
}

impl Debug for dyn ServiceTrait + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceTrait").finish()
    }
}

#[async_trait]
impl HandlerTrait for Handler {
    async fn new_game(&self, rows: i32, cols: i32, mines: i32) -> Result<Game, Error> {
        let g = self.service.new_game(rows, cols, mines).await?;
        Ok(map_to_model(g))
    }

    async fn get_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        let g = self.service.get_game(game_id).await?;
        Ok(map_to_model(g))
    }

    async fn pause_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        let g = self.service.pause_game(game_id).await?;
        Ok(map_to_model(g))
    }

    async fn resume_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        let g = self.service.resume_game(game_id).await?;
        Ok(map_to_model(g))
    }

    async fn mark_as_flag(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error> {
        let g = self.service.mark_as_flag(game_id, point).await?;
        Ok(map_to_model(g))
    }

    async fn mark_as_question(
        &self,
        game_id: uuid::Uuid,
        point: (i32, i32),
    ) -> Result<Game, Error> {
        let g = self.service.mark_as_question(game_id, point).await?;
        Ok(map_to_model(g))
    }

    async fn reveal(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error> {
        let g = self.service.reveal(game_id, point).await?;
        Ok(map_to_model(g))
    }
}

impl Handler {
    pub fn new<T>(service: T) -> Self
    where
        T: ServiceTrait + Send + Sync + 'static,
    {
        Self {
            service: Arc::new(service),
        }
    }

    pub fn router(&self) -> Router {
        let router = Router::new()
            .route("/api/v1/games/:id/state/", get(state_handler))
            .route("/api/v1/games/new/", post(new_handler))
            .route("/api/v1/games/:id/pause/", post(pause_handler))
            .route("/api/v1/games/:id/resume/", post(resume_handler))
            .route("/api/v1/games/:id/reveal/", post(reveal_handler))
            .route(
                "/api/v1/games/:id/mark_as_flag/",
                post(mark_as_flag_handler),
            )
            .route(
                "/api/v1/games/:id/mark_as_question/",
                post(mark_as_question_handler),
            )
            .with_state(self.clone());
        router
    }
}

async fn state_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.get_game(id).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn pause_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.pause_game(id).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn resume_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.resume_game(id).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn reveal_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
    extract::Json(at): extract::Json<crate::handler::model::At>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.reveal(id, (at.y, at.x)).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            Err(Error::InvalidPoint { point: _ }) => {
                (StatusCode::BAD_REQUEST, Json(Game::default()))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn mark_as_flag_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
    extract::Json(at): extract::Json<crate::handler::model::At>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.mark_as_flag(id, (at.y, at.x)).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            Err(Error::InvalidPoint { point: _ }) => {
                (StatusCode::BAD_REQUEST, Json(Game::default()))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn mark_as_question_handler(
    handler: State<Handler>,
    Path(params): Path<HashMap<String, String>>,
    extract::Json(at): extract::Json<crate::handler::model::At>,
) -> (StatusCode, Json<Game>) {
    match get_id(params) {
        Some(id) => match handler.mark_as_question(id, (at.y, at.x)).await {
            Ok(g) => (StatusCode::OK, Json(g)),
            Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
            Err(Error::InvalidPoint { point: _ }) => {
                (StatusCode::BAD_REQUEST, Json(Game::default()))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
        },
        None => (StatusCode::BAD_REQUEST, Json(Game::default())),
    }
}

async fn new_handler(
    handler: State<Handler>,
    extract::Json(params): extract::Json<crate::handler::model::Create>,
) -> (StatusCode, Json<Game>) {
    if params.rows < 1
        || params.columns < 1
        || params.mines < 1
        || params.mines >= params.rows * params.columns
    {
        return (StatusCode::BAD_REQUEST, Json(Game::default()));
    }
    match handler
        .new_game(params.rows, params.columns, params.mines)
        .await
    {
        Ok(g) => (StatusCode::OK, Json(g)),
        Err(Error::NotFound { id: _ }) => (StatusCode::NOT_FOUND, Json(Game::default())),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Game::default())),
    }
}

fn get_id(params: HashMap<String, String>) -> Option<uuid::Uuid> {
    match params.get("id") {
        Some(id) => match uuid::Uuid::parse_str(id) {
            Ok(uuid) => Some(uuid),
            Err(_) => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::MockServiceTrait;

    #[tokio::test]
    async fn paused_game_when_pause_game_then_returns_game() {
        let mut service = MockServiceTrait::new();
        service
            .expect_pause_game()
            .returning(|game_id: uuid::Uuid| {
                let mut game = crate::service::Game::new(4, 4, 1);
                game.id = game_id;
                Ok(game)
            });

        let handler = &Handler::new(service) as &dyn HandlerTrait;
        let game_id = uuid::Uuid::new_v4();
        assert_eq!(
            handler.pause_game(game_id).await.unwrap().id,
            game_id.to_string()
        );
    }
}
