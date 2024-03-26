use axum::middleware::from_fn_with_state;

use crate::http::{middleware::auth::auth, ApiState};

mod login;
mod logout;
mod signup;

pub fn router(api_state: ApiState) -> axum::Router<ApiState> {
    axum::Router::new()
        .route("/logout", axum::routing::delete(logout::delete))
        .route_layer(from_fn_with_state(api_state.clone(), auth))
        .route("/signup", axum::routing::post(signup::post))
        .route("/login", axum::routing::post(login::post))
}
