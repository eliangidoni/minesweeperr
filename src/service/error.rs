use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Database(#[from] tokio_postgres::error::Error),
    #[error("refinery error")]
    Refinery(#[from] refinery::Error),
    #[error("not found id {id:?}")]
    NotFound { id: String },
    #[error("invalid point {point:?}")]
    InvalidPoint { point: (i32, i32) },
}
