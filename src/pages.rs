use askama::Template;
use axum::http::StatusCode;

use crate::errors::{self, Error};

#[derive(Debug, Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

async fn error() -> ErrorTemplate {
    ErrorTemplate {
        message: "An Error Happened".to_string(),
    }
}

pub type ErrorResponse = (StatusCode, ErrorTemplate);

impl From<Error> for ErrorResponse {
    fn from(value: errors::Error) -> Self {
        match value {
            Error::MongoDB(_) => (value.into(), ErrorTemplate { message: String::from("Internal Server Error") }),
            Error::WrongSize => (value.into(), ErrorTemplate { message: String::from("Wrong Size") }),
        }
    }
}
