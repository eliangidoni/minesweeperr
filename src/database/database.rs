use crate::database::model::map_from_model;
use crate::database::model::map_to_model;
use crate::service::DatabaseTrait;
use crate::service::{Error, Game};
use axum::async_trait;
use std::sync::Arc;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn insert(&self, game: Game) -> Result<(), Error> {
        let value = map_to_model(game);
        self.client.execute(
            r##"
                INSERT INTO games
                    (id, created, updated, title, board, player_board, state, duration_seconds, elapsed_seconds, score, resumed_timestamp) 
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "##,
            &[&value.id, &value.created, &value.updated, &value.title, &value.board, &value.player_board, &value.state, &value.duration_seconds, &value.elapsed_seconds, &value.score, &value.resumed_timestamp])
            .await?;
        Result::Ok(())
    }

    async fn update(&self, game: Game) -> Result<(), Error> {
        let value = map_to_model(game);
        self.client.execute(
            r##"
                UPDATE games SET
                    (created, updated, title, board, player_board, state, duration_seconds, elapsed_seconds, score, resumed_timestamp)
                    = ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                WHERE
                    id = $11
            "##,
            &[&value.created, &value.updated, &value.title, &value.board, &value.player_board, &value.state, &value.duration_seconds, &value.elapsed_seconds, &value.score, &value.resumed_timestamp, &value.id])
            .await?;
        Result::Ok(())
    }

    async fn get(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        let rows = self.client.query(
            r##"
                SELECT
                    id, created, updated, title, board, player_board, state, duration_seconds, elapsed_seconds, score, resumed_timestamp
                FROM
                    games
                WHERE
                    id = $1
            "##,
            &[&game_id])
            .await?;
        if rows.is_empty() {
            return Result::Err(Error::NotFound {
                id: game_id.to_string(),
            });
        }
        Result::Ok(map_from_model(crate::database::model::Game {
            id: rows[0].get("id"),
            created: rows[0].get("created"),
            updated: rows[0].get("updated"),
            title: rows[0].get("title"),
            board: rows[0].get("board"),
            player_board: rows[0].get("player_board"),
            state: rows[0].get("state"),
            duration_seconds: rows[0].get("duration_seconds"),
            elapsed_seconds: rows[0].get("elapsed_seconds"),
            score: rows[0].get("score"),
            resumed_timestamp: rows[0].get("resumed_timestamp"),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    client: Arc<tokio_postgres::Client>,
}

impl Database {
    pub fn new(client: tokio_postgres::Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
    pub async fn run_migrations(client: &mut tokio_postgres::Client) -> Result<(), Error> {
        embedded::migrations::runner().run_async(client).await?;
        Result::Ok(())
    }
}
