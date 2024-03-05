use askama::Template;
use axum::{routing::get, Router};

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
        .route("/", get(index))
}

#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate {
    title: String,
    content: String,
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        title: String::from("Rentry"),
        content: String::from("This is the homepage"),
    }
}