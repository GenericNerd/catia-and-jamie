use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

use crate::http::{ResponseError, ResponseResult};

pub async fn method(request: Request, next: Next) -> ResponseResult<impl IntoResponse> {
    let method = request.method().to_string();
    let response = next.run(request).await;
    if response.status() == StatusCode::METHOD_NOT_ALLOWED {
        return Err(ResponseError {
            status: 405,
            message: Some(format!("Method {method} is not allowed")),
        });
    }
    Ok(response)
}
