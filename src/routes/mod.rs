use axum::Router;

use crate::routes::{api::api_routes, frontend::frontend_routes};

pub mod api;
pub mod frontend;

/// Configures and returns the global `Router` for the application.
///
/// This function combines all route sub-routers from different modules,
/// such as API and frontend routes, into a single global router. It allows
/// for a centralized routing setup that is easy to manage and expand.
///
/// Returns:
/// - `Router`: The fully configured global router.
pub fn configure_routes() -> Router {
    let api_routes = api_routes();
    let frontend_routes = frontend_routes();

    Router::new()
        .nest("/api", api_routes)
        .merge(frontend_routes)
}