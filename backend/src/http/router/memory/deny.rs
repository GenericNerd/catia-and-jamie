use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;

use crate::http::{ApiState, ExtractorRejection, ResponseError, ResponseResult};

#[derive(serde::Deserialize)]
pub struct DenyPayload {
    pub memory_id: i64,
}

pub async fn delete(
    State(state): State<ApiState>,
    WithRejection(Json(payload), _): WithRejection<Json<DenyPayload>, ExtractorRejection>,
) -> ResponseResult<impl IntoResponse> {
    let memory_url = sqlx::query!(
        "SELECT url FROM memories WHERE memory_id = $1",
        payload.memory_id
    )
    .fetch_one(&state.database_pool)
    .await?
    .url;

    if let Err(err) = std::fs::remove_file(memory_url) {
        return Err(ResponseError {
            status: 500,
            message: Some(format!("Failed to delete memory: {}", err)),
        });
    }

    sqlx::query!(
        "DELETE FROM memories WHERE memory_id = $1",
        payload.memory_id
    )
    .execute(&state.database_pool)
    .await?;

    Ok(())
}
