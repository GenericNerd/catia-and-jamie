use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    extract::WithRejection,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::http::{ApiState, ExtractorRejection, ResponseResult};

pub async fn auth(
    State(state): State<ApiState>,
    WithRejection(TypedHeader(auth), _): WithRejection<
        TypedHeader<Authorization<Bearer>>,
        ExtractorRejection,
    >,
    mut request: Request,
    next: Next,
) -> ResponseResult<impl IntoResponse> {
    let user_id = match sqlx::query!(
        "SELECT user_id FROM user_sessions WHERE token = $1",
        auth.token()
    )
    .fetch_optional(&state.database_pool)
    .await
    {
        Ok(Some(row)) => row.user_id,
        _ => return Ok("Unauthorized".into_response()),
    };

    request.extensions_mut().insert(user_id);
    let response = next.run(request).await;
    Ok(response)
}
