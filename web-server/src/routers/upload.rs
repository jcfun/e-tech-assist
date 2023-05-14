use crate::{config::init::get_cfg, handlers::upload::*};
use axum::{extract::DefaultBodyLimit, routing::post, Router};

pub fn uploads_routes() -> Router {
    Router::new().route(
        "/image",
        post(image_upload).layer(DefaultBodyLimit::max(get_cfg().mime.image.size)),
    )
}
