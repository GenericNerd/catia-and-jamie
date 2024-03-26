use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;

use crate::http::{ApiState, ExtractorRejection, ResponseResult};

#[derive(serde::Deserialize)]
pub struct ApprovePayload {
    pub memory_id: i64,
}

pub async fn put(
    State(state): State<ApiState>,
    WithRejection(Json(payload), _): WithRejection<Json<ApprovePayload>, ExtractorRejection>,
) -> ResponseResult<impl IntoResponse> {
    sqlx::query!(
        "UPDATE memories SET approved = TRUE WHERE memory_id = $1",
        payload.memory_id
    )
    .execute(&state.database_pool)
    .await?;

    Ok(())
}
