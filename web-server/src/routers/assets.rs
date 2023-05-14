use axum::{routing::get_service, Router};
use tower_http::services::fs::ServeDir;
pub fn assets_routes() -> Router {
    Router::new()
        .nest_service("/images", get_service(ServeDir::new("assets/images")))
        .nest_service("/files", get_service(ServeDir::new("./assets/files")))
}
