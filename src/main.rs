// USE "JetBrains Mono"

use axum::Extension;
use routes::configure_routes;
use state::AppState;

mod db;
mod errors;
mod routes;
mod state;
mod validators;
mod services;

// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {

    env_logger::init();

    let app_state = AppState::new("./database");
    let app_routes = configure_routes()
        .layer(Extension(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_routes).await.unwrap();
}
