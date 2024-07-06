use serde::{Deserialize, Serialize};

use crate::service;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub state: String,
    pub board_view: Vec<Vec<char>>,
    pub duration_seconds: i32,
    pub elapsed_seconds: i32,
    pub score: i32,
    pub resumed_timestamp: Option<time::OffsetDateTime>,
}

pub(super) fn map_to_model(g: service::Game) -> Game {
    Game {
        id: g.id.to_string(),
        title: g.title.clone(),
        state: g.state.to_string(),
        board_view: g.get_board_view(),
        duration_seconds: g.duration_seconds,
        elapsed_seconds: g.elapsed_seconds,
        score: g.score,
        resumed_timestamp: g.resumed_timestamp,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create {
    pub rows: i32,
    pub columns: i32,
    pub mines: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct At {
    pub x: i32,
    pub y: i32,
}
