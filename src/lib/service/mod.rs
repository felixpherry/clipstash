pub mod action;
pub mod ask;

use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("permission denied: {0}")]
    PermissionError(String),
}

impl From<DataError> for ServiceError {
    fn from(e: DataError) -> Self {
        match e {
            DataError::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other)),
        }
    }
}
