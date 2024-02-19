use axum::response::IntoResponse;

use crate::http::ResponseResult;

pub async fn delete() -> ResponseResult<impl IntoResponse> {
    Ok(())
}
