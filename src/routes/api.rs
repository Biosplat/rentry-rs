use askama_axum::IntoResponse;
use axum::{
    extract,
    http::StatusCode,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use log::error;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::{
    db::{Database, DocumentHash, DocumentRecord, SlugRecord},
    errors::Error,
    services::{create_paste, edit_paste},
    state::AppState,
    validators::{is_invalid_document, is_invalid_edit_code, is_invalid_slug},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonError {
    pub message: String,
}

pub struct JsonErrorResponse(StatusCode, String);

impl IntoResponse for JsonErrorResponse {
    fn into_response(self) -> askama_axum::Response {
        (
            self.0,
            Json(JsonError {
                message: self.1.into(),
            }),
        )
            .into_response()
    }
}

impl From<Error> for JsonErrorResponse {
    fn from(e: Error) -> Self {
        error!("Internal server error: {e}");

        JsonErrorResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".into(),
        )
    }
}

/// Constructs and returns a router with all API endpoints for managing pastes.
///
/// This configuration facilitates operations such as creation, edition, deletion,
/// and retrieval of paste entries, with additional utility for rendering markdown content.
/// It is designed for seamless integration into the application's global routing structure,
/// emphasizing modular and clear route definitions.
///
/// Returns:
/// - `Router`: A router meticulously configured with dedicated routes for paste management
///   and markdown rendering, supporting a comprehensive API interface.
pub fn api_routes() -> Router {
    Router::new()
        .merge(paste_routes())
        .route("/markdown/render", post(render_markdown_handler))
}

/// Constructs a router dedicated to paste management operations.
///
/// This router includes endpoints for creating, editing, deleting, and
/// retrieving paste entries, alongside a specialized endpoint for fetching
/// the HTML representation of a specific paste. It embodies a focused subset
/// of the broader API functionality, prioritizing clarity and functional segregation.
///
/// Returns:
/// - `Router`: A router elegantly configured with routes specifically tailored
///   for paste-related operations, ensuring an organized and intuitive API structure.
pub fn paste_routes() -> Router {
    Router::new()
        .route("/pastes", post(create_paste_handler)) // post a new paste
        .route("/pastes/:id", put(edit_paste_handler)) // edit an existing paste
        .route("/pastes/:id", delete(delete_paste_handler)) // delete an existing paste
        .route("/pastes/:id", get(get_paste_handler)) // get a specific paste
        .route("/pastes/:id/html", get(get_paste_html_handler)) // get the html for the specific paste
}

pub fn check_slug_format(slug: &str) -> Result<(), JsonErrorResponse> {
    if is_invalid_slug(slug) {
        return Err(JsonErrorResponse(
            StatusCode::BAD_REQUEST,
            "slug must be between 4 and 16 characters and ascii alphanumeric".into(),
        ));
    }
    Ok(())
}

pub fn check_edit_code_format(edit_code: &str) -> Result<(), JsonErrorResponse> {
    if is_invalid_edit_code(edit_code) {
        return Err(JsonErrorResponse(
            StatusCode::BAD_REQUEST,
            "edit code must be between 4 and 32 characters and valid ascii".into(),
        ));
    }
    Ok(())
}

pub fn check_document(contents: &str) -> Result<(), JsonErrorResponse> {
    if is_invalid_document(contents) {
        return Err(JsonErrorResponse(
            StatusCode::BAD_REQUEST,
            "document must be 200,000 or less bytes".into(),
        ));
    }
    Ok(())
}

pub fn check_slug_access(
    db: &Database,
    slug: &str,
    edit_code: &str,
) -> Result<(), JsonErrorResponse> {
    let record = check_slug_exists(db, slug)?;
    if record.edit_code == edit_code {
        Ok(())
    } else {
        Err(JsonErrorResponse(
            StatusCode::FORBIDDEN,
            "You do not have permission to edit this document".into(),
        ))
    }

    // match db.get_slug(slug)? {
    //     Some(slug_record) if slug_record.edit_code == edit_code => Ok(()),
    //     Some(_) => Err(JsonErrorResponse(
    //         StatusCode::FORBIDDEN,
    //         "You do not have permission to edit this document".into(),
    //     )),
    //     None => Err(JsonErrorResponse(
    //         StatusCode::NOT_FOUND,
    //         "the requested slug was not found".into(),
    //     )),
    // }
}

pub fn check_slug_exists(db: &Database, slug: &str) -> Result<SlugRecord, JsonErrorResponse> {
    match db.get_slug(slug)? {
        Some(slug_record) => Ok(slug_record),
        None => Err(JsonErrorResponse(
            StatusCode::NOT_FOUND,
            "the requested slug was not found".into(),
        )),
    }
}

pub fn check_document_exists(
    db: &Database,
    hash: &DocumentHash,
) -> Result<DocumentRecord, JsonErrorResponse> {
    match db.get_document(hash)? {
        Some(doc_record) => Ok(doc_record),
        None => Err(JsonErrorResponse(
            StatusCode::NOT_FOUND,
            "the requested document was not found".into(),
        )),
    }
}

/// Handles the creation of a new paste, receiving paste details as JSON.
/// Returns a unique identifier for the newly created paste.
async fn create_paste_handler(
    state: Extension<AppState>,
    request: extract::Json<CreatePaste>,
) -> Result<Json<CreatePasteResponse>, JsonErrorResponse> {
    if let Some(ref slug) = request.custom_slug {
        check_slug_format(slug)?;

        if let Some(_) = state.db.get_slug(slug)? {
            return Err(JsonErrorResponse(
                StatusCode::CONFLICT,
                "specified slug is taken".into(),
            ));
        }
    }

    if let Some(ref edit_code) = request.edit_code {
        check_edit_code_format(edit_code)?;
    }

    check_document(&request.content)?;

    let slug = request.custom_slug.clone().unwrap_or(nanoid!(8));
    let edit_code = request.edit_code.clone().unwrap_or(nanoid!(16));

    // match create_paste(&state.db, &slug, &edit_code, &request.content) {
    //     Ok(_) => Ok(Json(CreatePasteResponse { slug, edit_code })),
    //     Err(e) => {
    //         error!("Internal server error: {e}");

    //         Err(JsonErrorResponse(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "internal server error".into(),
    //         ))
    //     }
    // }
    create_paste(&state.db, &slug, &edit_code, &request.content)?;
    Ok(Json(CreatePasteResponse { slug, edit_code }))
}

/// Edits an existing paste identified by a unique ID, updating it with new content provided as JSON.
/// Returns confirmation of the edit operation.
async fn edit_paste_handler(
    state: Extension<AppState>,
    slug: extract::Path<String>,
    request: extract::Json<EditPaste>,
) -> Result<Json<EditPasteResponse>, JsonErrorResponse> {
    let slug = slug.as_str();
    check_slug_access(&state.db, slug, &request.edit_code)?;

    // match edit_paste(&state.db, slug, &request.edit_code, &request.content) {
    //     Ok(_) => Ok(Json(EditPasteResponse {})),
    //     Err(e) => {
    //         error!("Internal server error: {e}");

    //         Err(JsonErrorResponse(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "internal server error".into(),
    //         ))
    //     }
    // }

    edit_paste(&state.db, slug, &request.edit_code, &request.content)?;
    Ok(Json(EditPasteResponse {}))

    // match state.db.get_slug(slug) {
    //     Ok(Some(slug_record)) if slug_record.edit_code == request.edit_code => {
    //         match edit_paste(&state.db, slug, &slug_record.edit_code, &request.content) {
    //             Ok(_) => Ok(Json(EditPasteResponse {})),
    //             Err(e) => {
    //                 error!("Internal server error: {e}");
    //                 Err(JsonErrorResponse(
    //                     StatusCode::INTERNAL_SERVER_ERROR,
    //                     "internal server error".into(),
    //                 ))
    //             }
    //         }
    //     }
    //     Ok(Some(_)) => Err(JsonErrorResponse(
    //         StatusCode::FORBIDDEN,
    //         "You do not have permission to edit this document".into(),
    //     )),
    //     Ok(None) => Err(JsonErrorResponse(
    //         StatusCode::NOT_FOUND,
    //         "the requested slug was not found".into(),
    //     )),
    //     Err(e) => {
    //         error!("Internal server error: {e}");
    //         Err(JsonErrorResponse(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "internal server error".into(),
    //         ))
    //     }
    // }
}

/// Deletes a specific paste identified by a unique ID.
/// Returns a confirmation of the deletion operation.
async fn delete_paste_handler(
    state: Extension<AppState>,
    slug: extract::Path<String>,
    request: extract::Json<DeletePaste>,
) -> Result<Json<DeletePasteResponse>, JsonErrorResponse> {
    let slug = slug.as_str();
    check_slug_access(&state.db, slug, &request.edit_code)?;

    state.db.remove_slug(slug)?;
    Ok(Json(DeletePasteResponse {}))
}

/// Retrieves the content of a specific paste by its unique ID.
/// Returns the paste's content and metadata.
async fn get_paste_handler(
    state: Extension<AppState>,
    slug: extract::Path<String>,
) -> Result<Json<GetPasteResponse>, JsonErrorResponse> {
    let slug = slug.as_str();

    let slug_record = check_slug_exists(&state.db, slug)?;
    let doc_record = check_document_exists(&state.db, &slug_record.document_hash)?;

    Ok(Json(GetPasteResponse {
        contents: doc_record.content,
        created: doc_record.created,
    }))
}

/// Retrieves the HTML-rendered content of a specific paste by its unique ID.
/// Useful for displaying formatted paste content in a web interface.
async fn get_paste_html_handler(_state: Extension<AppState>, _id: extract::Path<String>) {
    todo!()
}

/// Converts markdown content provided in the request body to HTML.
/// Returns the rendered HTML content for preview or display purposes.
async fn render_markdown_handler(_request: extract::Json<RenderMarkdown>) {
    todo!()
}

/// Represents the input structure for creating a new paste.
#[derive(Debug, Deserialize)]
pub struct CreatePaste {
    pub custom_slug: Option<String>,
    pub edit_code: Option<String>,
    pub content: String,
}

/// Represents the response structure for creating a new paste.
#[derive(Debug, Serialize)]
pub struct CreatePasteResponse {
    // Fields to be determined
    slug: String,
    edit_code: String,
}

/// Represents the input structure for editing an existing paste.
#[derive(Debug, Deserialize)]
pub struct EditPaste {
    pub edit_code: String,
    pub content: String,
    // Fields to be determined
}

/// Represents the response structure for editing a paste.
#[derive(Debug, Serialize)]
pub struct EditPasteResponse {
    // Fields to be determined
}

/// Represents the input structure for deleting a paste.
#[derive(Debug, Deserialize)]
pub struct DeletePaste {
    pub edit_code: String,
    // Fields to be determined
}

/// Represents the response structure for a successful paste deletion.
#[derive(Debug, Serialize)]
pub struct DeletePasteResponse {
    // Fields to be determined
}

/// Represents the response structure containing the content and metadata of a requested paste.
#[derive(Debug, Serialize)]
pub struct GetPasteResponse {
    contents: String,
    created: DateTime<Utc>, // Fields to be determined
}

/// Represents the response structure containing the HTML-rendered content of a requested paste.
#[derive(Debug, Serialize)]
pub struct GetPasteHtmlResponse {
    // Fields to be determined
}

/// Represents the input structure for converting markdown to HTML.
#[derive(Debug, Deserialize)]
pub struct RenderMarkdown {
    // Fields to be determined
}

/// Represents the response structure containing the HTML-rendered markdown content.
#[derive(Debug, Serialize)]
pub struct RenderMarkdownResponse {
    // Fields to be determined
}
