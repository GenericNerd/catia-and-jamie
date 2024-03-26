use axum::{extract::State, response::IntoResponse};
use axum_extra::{
    extract::WithRejection,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::http::{ApiState, ExtractorRejection, ResponseResult};

pub async fn delete(
    state: State<ApiState>,
    WithRejection(TypedHeader(auth), _): WithRejection<
        TypedHeader<Authorization<Bearer>>,
        ExtractorRejection,
    >,
) -> ResponseResult<impl IntoResponse> {
    sqlx::query!("DELETE FROM user_sessions WHERE token = $1", auth.token())
        .execute(&state.database_pool)
        .await?;

    Ok(())
}
