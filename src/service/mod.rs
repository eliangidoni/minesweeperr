mod service;
pub use self::service::DatabaseTrait;
pub use self::service::Service;
pub use self::service::ServiceTrait;

#[cfg(test)]
pub use self::service::MockServiceTrait;

mod model;
pub use self::model::Game;
pub use self::model::State;

mod error;
pub use self::error::Error;
