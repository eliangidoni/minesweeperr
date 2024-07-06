use ::axum_test::TestServer;
mod common;
use minesweeperrust::handler;
use minesweeperrust::service::{DatabaseTrait, Game};
use serde_json::json;
use serial_test::serial;

#[tokio::test]
async fn new_request_when_handle_new_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    assert_eq!(game.board_view.len(), 4);
    assert_eq!(game.board_view[0].len(), 4);
}

#[tokio::test]
async fn invalid_new_request_when_handle_new_then_returns_badrequest() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 0,
            columns: 0,
            mines: 0
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn state_request_when_handle_state_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .get(format!("/api/v1/games/{}/state/", game.id).as_str())
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn absent_state_request_when_handle_state_then_returns_notfound() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let id = "00000000-0000-0000-0000-000000000000";
    let resp = server
        .get(format!("/api/v1/games/{}/state/", id).as_str())
        .await;
    resp.assert_status_not_found();
}

#[tokio::test]
async fn pause_request_when_handle_pause_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/pause/", game.id).as_str())
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn resume_request_when_handle_resume_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/resume/", game.id).as_str())
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn reveal_request_when_handle_reveal_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/reveal/", game.id).as_str())
        .json(&json!(handler::At { x: 1, y: 1 }))
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn invalid_reveal_request_when_handle_reveal_then_returns_badrequest() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/reveal/", game.id).as_str())
        .json(&json!(handler::At { x: -1, y: 1 }))
        .await;
    resp2.assert_status_bad_request();
}

#[tokio::test]
async fn question_request_when_handle_question_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/mark_as_question/", game.id).as_str())
        .json(&json!(handler::At { x: 1, y: 1 }))
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn invalid_question_request_when_handle_question_then_returns_badrequest() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/mark_as_question/", game.id).as_str())
        .json(&json!(handler::At { x: 1, y: 100 }))
        .await;
    resp2.assert_status_bad_request();
}

#[tokio::test]
async fn flag_request_when_handle_flag_then_returns_ok() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/mark_as_flag/", game.id).as_str())
        .json(&json!(handler::At { x: 1, y: 1 }))
        .await;
    resp2.assert_status_ok();
    let game2: handler::Game = resp2.json();
    assert_eq!(game2.id, game.id);
}

#[tokio::test]
async fn invalid_flag_request_when_handle_flag_then_returns_badrequest() {
    let (router, _) = common::setup().await;
    let server = TestServer::new(router).unwrap();
    let resp = server
        .post(format!("/api/v1/games/new/").as_str())
        .json(&json!(handler::Create {
            rows: 4,
            columns: 4,
            mines: 1
        }))
        .await;
    resp.assert_status_ok();
    let game: handler::Game = resp.json();
    let resp2 = server
        .post(format!("/api/v1/games/{}/mark_as_flag/", game.id).as_str())
        .json(&json!(handler::At { x: 1, y: -1 }))
        .await;
    resp2.assert_status_bad_request();
}

#[tokio::test]
#[serial]
async fn insert_game_when_database_get_returns_game() {
    let database = common::setup_database().await;

    let mut game = Game::new(4, 4, 1);
    game.title = "Hello from Dependency".to_string();
    assert!(database.insert(game.clone()).await.is_ok());

    let read_game = database.get(game.id).await.unwrap();
    assert_eq!(game.title, read_game.title);

    game.title = "Hello from Dependency and test".to_string();
    assert!(database.update(game.clone()).await.is_ok());

    let read_game = database.get(game.id).await.unwrap();
    assert_eq!(game.title, read_game.title);
}
