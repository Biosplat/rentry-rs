use askama::Template;
use axum::{http::Uri, routing::{get, get_service}, Router};
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
        .route("/", get(index))
        .fallback(not_found)
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

async fn index() -> IndexTemplate {
    IndexTemplate {
        title: String::from("Rentry"),
    }
}

async fn not_found(uri: Uri) -> NotFoundTemplate {
    NotFoundTemplate {
        address: uri.to_string(),
    }
}

#[derive(Template)]
#[template(path="404.html")]
struct NotFoundTemplate {
    address: String,
}