use askama_axum::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::Serialize;
use thiserror::Error;


/// A custom error enum to encapsulate various errors that can occur within the application.
///
/// # Variants:
///
/// - `Sled`: Wraps errors originating from the `sled` database interactions.
/// - `Bincode`: Encapsulates serialization and deserialization errors from the `bincode` crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Sled Error: {0}")]
    Sled(#[from] sled::Error),

    #[error("Bincode Error: {0}")]
    Bincode(#[from] bincode::Error)
}

/// An enumeration to classify types of API errors.
///
/// This allows for a structured approach to categorizing errors at the API layer,
/// enabling consistent HTTP status code responses based on the error type.
///
/// # Variants:
///
/// - `Backend`: Represents errors related to server-side issues, such as database failures or internal bugs.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ApiErrorType {
    Backend,
}

/// Implements a conversion from `ApiErrorType` to `StatusCode`.
///
/// This conversion facilitates mapping an API error type to an appropriate HTTP status code,
/// ensuring that API responses align with standard HTTP semantics for error conditions.
impl Into<StatusCode> for ApiErrorType {
    fn into(self) -> StatusCode {
        match self {
            ApiErrorType::Backend => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// A structured representation of an API error.
///
/// This struct is designed to be serialized into JSON and returned to the client
/// in the event of an error, providing clear and actionable error information.
///
/// # Fields:
///
/// - `error_type`: The category/type of the error, influencing the HTTP status code.
/// - `message`: A human-readable message describing the error.
#[derive(Debug, Serialize)]
pub struct ApiError {
    error_type: ApiErrorType,
    message: String,
}

/// Implements conversion from the internal `Error` enum to `ApiError`.
///
/// This conversion encapsulates the logic for mapping internal application errors
/// to their corresponding API-level representation, including determining the error message
/// and categorizing the error type.
impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        match value {
            Error::Sled(_) => Self {
                error_type: ApiErrorType::Backend,
                message: format!("internal database error"),
            },
            Error::Bincode(_) => Self {
                error_type: ApiErrorType::Backend,
                message: format!("internal serialization error"),
            },
        }
    }
}

/// Implements conversion from `ApiError` to an Axum `Response`.
///
/// This enables `ApiError` instances to be seamlessly converted into HTTP responses
/// that can be returned from Axum route handlers, facilitating an ergonomic error handling
/// mechanism within the web application framework.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status: StatusCode = self.error_type.into();
        (status, Json(self)).into_response()
    }
}

/// A type alias for API responses.
///
/// This alias represents the result of an operation that can either succeed, returning
/// a serialized JSON payload, or fail, returning an `ApiError`.
///
/// The success path wraps the response data in `Json<T>`, where `T` is the data type
/// meant to be serialized and sent to the client.
pub type ApiResponse<T> = Result<Json<T>, ApiError>;