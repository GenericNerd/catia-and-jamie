use axum::{handler::HandlerWithoutStateExt, response::IntoResponse};
use tower_http::services::ServeDir;

use super::{not_found, ApiState, ResponseResult};

mod auth;

async fn get() -> ResponseResult<impl IntoResponse> {
    Ok(axum::Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION")
    })))
}

pub fn api_router(api_state: ApiState) -> axum::Router {
    axum::Router::new().nest(
        "/api",
        axum::Router::new()
            .route("/", axum::routing::get(get))
            .nest("/auth", auth::router(api_state.clone()))
            .nest_service(
                "/assets",
                ServeDir::new("assets").not_found_service(not_found.into_service()),
            )
            .with_state(api_state),
    )
}
