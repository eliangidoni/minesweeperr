use crate::service::State;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, Json, ToSql};

#[derive(Debug, Clone, FromSql, ToSql)]
pub(super) struct Game {
    pub id: uuid::Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub title: String,
    pub board: Json<Board>,
    pub player_board: Json<Board>,
    pub state: i32,
    pub duration_seconds: i32,
    pub elapsed_seconds: i32,
    pub score: i32,
    pub resumed_timestamp: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Board {
    pub rows: Vec<Vec<char>>,
}

pub(super) fn state_to_i32(state: State) -> i32 {
    match state {
        State::New => 0,
        State::Started => 1,
        State::Paused => 2,
        State::Timeout => 3,
        State::Won => 4,
        State::Lost => 5,
    }
}

pub(super) fn i32_to_state(state: i32) -> State {
    match state {
        0 => State::New,
        1 => State::Started,
        2 => State::Paused,
        3 => State::Timeout,
        4 => State::Won,
        5 => State::Lost,
        _ => panic!("Invalid state"),
    }
}

pub(super) fn map_to_model(game: crate::service::Game) -> Game {
    Game {
        id: game.id,
        created: game.created,
        updated: game.updated,
        title: game.title,
        board: Json(Board { rows: game.board }),
        player_board: Json(Board {
            rows: game.player_board,
        }),
        state: state_to_i32(game.state),
        duration_seconds: game.duration_seconds,
        elapsed_seconds: game.elapsed_seconds,
        score: game.score,
        resumed_timestamp: game.resumed_timestamp,
    }
}

pub(super) fn map_from_model(game: Game) -> crate::service::Game {
    crate::service::Game {
        id: game.id,
        created: game.created,
        updated: game.updated,
        title: game.title,
        board: game.board.0.rows,
        player_board: game.player_board.0.rows,
        state: i32_to_state(game.state),
        duration_seconds: game.duration_seconds,
        elapsed_seconds: game.elapsed_seconds,
        score: game.score,
        resumed_timestamp: game.resumed_timestamp,
    }
}
