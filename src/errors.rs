use axum::http::StatusCode;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("MongoDB Error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    
    #[error("wrong size")]
    WrongSize,
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::MongoDB(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::WrongSize => StatusCode::BAD_REQUEST,
        }
    }
}