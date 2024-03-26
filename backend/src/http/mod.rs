use axum::{extract::rejection::JsonRejection, response::IntoResponse};
use axum_extra::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod middleware;
mod router;

pub struct ResponseError {
    pub status: u16,
    pub message: Option<String>,
}

pub type ResponseResult<T> = Result<T, ResponseError>;

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::from_u16(self.status).unwrap(),
            axum::Json(serde_json::json!({
                "success": false,
                "message": self.message
            })),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for ResponseError {
    fn from(err: sqlx::Error) -> Self {
        ResponseError {
            status: 500,
            message: Some(err.to_string()),
        }
    }
}

impl From<std::io::Error> for ResponseError {
    fn from(err: std::io::Error) -> Self {
        ResponseError {
            status: 500,
            message: Some(err.to_string()),
        }
    }
}

impl From<image::ImageError> for ResponseError {
    fn from(err: image::ImageError) -> Self {
        ResponseError {
            status: 400,
            message: Some(err.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractorRejection {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    TypedHeaderRejection(#[from] TypedHeaderRejection),
}

impl IntoResponse for ExtractorRejection {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ExtractorRejection::JsonExtractorRejection(rejection) => {
                (rejection.status().as_u16(), rejection.body_text())
            }
            ExtractorRejection::TypedHeaderRejection(rejection) => {
                let reason = rejection.reason();
                match reason {
                    TypedHeaderRejectionReason::Missing => {
                        (400, format!("Missing {} header", rejection.name()))
                    }
                    TypedHeaderRejectionReason::Error(err) => (
                        400,
                        format!("{} on {} header", err.to_string(), rejection.name()),
                    ),
                    _ => (400, "Invalid header".to_string()),
                }
            }
        };

        ResponseError {
            status,
            message: Some(message),
        }
        .into_response()
    }
}

#[derive(Clone)]
pub struct ApiState {
    pub database_pool: sqlx::PgPool,
    pub openssl_cipher: openssl::symm::Cipher,
    pub openssl_encryption_key: &'static str,
    pub session_secret: &'static str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserTokenClaims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub iss: String,
}

impl UserTokenClaims {
    pub fn new(sub: i64) -> Self {
        let issued_at = time::OffsetDateTime::now_utc();
        let expires_at = issued_at.checked_add(time::Duration::days(7)).unwrap();
        Self {
            sub,
            exp: expires_at.unix_timestamp(),
            iat: issued_at.unix_timestamp(),
            nbf: issued_at.unix_timestamp(),
            iss: "caj".to_string(),
        }
    }
}

pub async fn not_found() -> impl IntoResponse {
    ResponseError {
        status: 404,
        message: Some("Not found".to_string()),
    }
    .into_response()
}

pub async fn get_app(
    port: u16,
    database_pool: sqlx::PgPool,
    openssl_encryption_key: String,
    session_secret: String,
) -> (axum::Router, tokio::net::TcpListener) {
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!("Failed to bind to port {port}: {err}");
            panic!("Failed to bind to port {port}");
        }
    };

    let api_context = ApiState {
        database_pool,
        openssl_cipher: openssl::symm::Cipher::aes_256_cbc(),
        openssl_encryption_key: Box::leak(openssl_encryption_key.into_boxed_str()),
        session_secret: Box::leak(session_secret.into_boxed_str()),
    };

    let app = router::api_router(api_context)
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([
                        axum::http::Method::GET,
                        axum::http::Method::POST,
                        axum::http::Method::PUT,
                        axum::http::Method::DELETE,
                        axum::http::Method::OPTIONS,
                        axum::http::Method::HEAD,
                    ])
                    .allow_origin(tower_http::cors::Any),
            ),
        )
        .layer(axum::middleware::from_fn(middleware::method::method))
        .fallback(not_found);

    (app, listener)
}
