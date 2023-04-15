use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any)
}
