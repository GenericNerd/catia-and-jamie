use axum::{extract::State, middleware::from_fn_with_state, response::IntoResponse, Json};
use axum_extra::{
    extract::WithRejection,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::http::{
    middleware::auth::auth, ApiState, ExtractorRejection, ResponseError, ResponseResult,
};

mod approve;
mod deny;
mod new;

pub async fn get(
    State(state): State<ApiState>,
    header: Option<WithRejection<TypedHeader<Authorization<Bearer>>, ExtractorRejection>>,
) -> ResponseResult<impl IntoResponse> {
    if let Some(header) = header {
        if sqlx::query!(
            "SELECT user_id FROM user_sessions WHERE token = $1",
            header.0.token()
        )
        .fetch_optional(&state.database_pool)
        .await?
        .is_none()
        {
            return Err(ResponseError {
                status: 401,
                message: Some("Unauthorized".to_string()),
            });
        }

        let memories = sqlx::query!("SELECT memory_id, table_name, url, approved FROM memories")
            .fetch_all(&state.database_pool)
            .await?
            .iter()
            .map(|memory| {
                serde_json::json!({
                    "memory_id": memory.memory_id,
                    "table_name": memory.table_name,
                    "url": memory.url,
                    "approved": memory.approved
                })
            })
            .collect::<Vec<serde_json::Value>>();

        return Ok(Json(serde_json::json!({
            "success": true,
            "memories": memories
        })));
    } else {
        let memories =
            sqlx::query!("SELECT memory_id, table_name, url FROM memories WHERE approved = TRUE")
                .fetch_all(&state.database_pool)
                .await?
                .iter()
                .map(|memory| {
                    serde_json::json!({
                        "memory_id": memory.memory_id,
                        "table_name": memory.table_name,
                        "url": memory.url,
                    })
                })
                .collect::<Vec<serde_json::Value>>();

        return Ok(Json(serde_json::json!({
            "success": true,
            "memories": memories
        })));
    };
}

pub fn router(api_state: ApiState) -> axum::Router<ApiState> {
    axum::Router::new()
        .route("/approve", axum::routing::put(approve::put))
        .route("/deny", axum::routing::delete(deny::delete))
        .layer(from_fn_with_state(api_state.clone(), auth))
        .route("/", axum::routing::get(get))
        .route("/new", axum::routing::post(new::post))
}
