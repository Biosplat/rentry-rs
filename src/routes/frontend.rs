use std::collections::HashMap;

use askama::Template;
use axum::{extract, http::Uri, routing::{get, get_service}, Router};
use tower_http::services::ServeDir;

/// Creates and returns a router for frontend-related routes.
///
/// This includes routes for serving HTML content, such as the homepage or
/// other static pages. It's designed to be part of the application's global
/// router setup.
///
/// Returns:
/// - `Router`: A router configured with frontend routes.
pub fn frontend_routes() -> Router {
    Router::new()
        .merge(static_routes())
        .route("/", get(index_handler))
        .route("/admin", get(admin_handler))
        .route("/p/:slug", get(paste_handler))
        .fallback(not_found_handler)
}

pub fn static_routes() -> Router {
    Router::new()
        .nest_service("/static", get_service(ServeDir::new("static")))
}

#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate {
    title: String,
}

async fn index_handler() -> IndexTemplate {
    IndexTemplate {
        title: String::from("Rentry"),
    }
}

async fn not_found_handler(uri: Uri) -> NotFoundTemplate {
    NotFoundTemplate {
        address: uri.to_string(),
    }
}

#[derive(Template)]
#[template(path="404.html")]
struct NotFoundTemplate {
    address: String,
}

#[derive(Template)]
#[template(path="admin.html")]
struct AdminTemplate {}

async fn admin_handler() -> AdminTemplate {
    AdminTemplate {}
}


#[derive(Template)]
#[template(path="preview.html")]
pub struct MarkdownPreview {
    slug: String,
    edit_code: Option<String>,
}

async fn paste_handler(slug: extract::Path<String>, query: extract::Query<HashMap<String, String>>) -> MarkdownPreview {
    let edit_code = query.get("edit_code").cloned();
    MarkdownPreview { slug: slug.0, edit_code }
}