use crate::service::{Error, Game};
use axum::async_trait;
#[cfg(test)]
use mockall::automock;

use super::State;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ServiceTrait {
    async fn new_game(&self, rows: i32, cols: i32, mines: i32) -> Result<Game, Error>;
    async fn get_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn pause_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn resume_game(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn mark_as_flag(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error>;
    async fn mark_as_question(&self, game_id: uuid::Uuid, point: (i32, i32))
        -> Result<Game, Error>;
    async fn reveal(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DatabaseTrait {
    async fn get(&self, game_id: uuid::Uuid) -> Result<Game, Error>;
    async fn insert(&self, value: Game) -> Result<(), Error>;
    async fn update(&self, value: Game) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct Service<T> {
    dependency: T,
}

#[async_trait]
impl<T> ServiceTrait for Service<T>
where
    T: DatabaseTrait + Sync,
{
    async fn new_game(&self, rows: i32, cols: i32, mines: i32) -> Result<Game, Error> {
        let g = Game::new(rows, cols, mines);
        self.dependency.insert(g.clone()).await?;
        Ok(g)
    }

    async fn get_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        self.dependency.get(game_id).await
    }

    async fn pause_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        self.dependency.get(game_id).await
    }

    async fn resume_game(&self, game_id: uuid::Uuid) -> Result<Game, Error> {
        self.dependency.get(game_id).await
    }

    async fn mark_as_flag(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error> {
        let mut g = self.dependency.get(game_id).await?;
        match g.new_point(point) {
            Some(p) => {
                g.mark_flag_at(p);
                self.dependency.update(g.clone()).await?;
                Ok(g)
            }
            None => Err(Error::InvalidPoint { point: point }),
        }
    }

    async fn mark_as_question(
        &self,
        game_id: uuid::Uuid,
        point: (i32, i32),
    ) -> Result<Game, Error> {
        let mut g = self.dependency.get(game_id).await?;
        match g.new_point(point) {
            Some(p) => {
                g.mark_question_at(p);
                self.dependency.update(g.clone()).await?;
                Ok(g)
            }
            None => Err(Error::InvalidPoint { point: point }),
        }
    }

    async fn reveal(&self, game_id: uuid::Uuid, point: (i32, i32)) -> Result<Game, Error> {
        let mut g = self.dependency.get(game_id).await?;
        match g.new_point(point) {
            Some(p) => {
                g.reveal_at(p);
                if g.is_mine_at(p) {
                    g.state = State::Lost;
                } else if g.is_all_revealed() {
                    g.state = State::Won;
                }
                self.dependency.update(g.clone()).await?;
                Ok(g)
            }
            None => Err(Error::InvalidPoint { point: point }),
        }
    }
}

impl<T> Service<T>
where
    T: DatabaseTrait,
{
    pub fn new(dep: T) -> Self {
        Self { dependency: dep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn existing_game_when_pause_game_returns_game() {
        let mut dep = MockDatabaseTrait::new();
        let uid = uuid::Uuid::new_v4();
        dep.expect_get().returning(|id: uuid::Uuid| {
            let mut g = Game::new(4, 4, 1);
            g.id = id;
            Ok(g)
        });
        let service = &Service::new(dep) as &dyn ServiceTrait;
        assert_eq!(service.pause_game(uid).await.unwrap().id, uid);
    }
}
