use argon2::{PasswordHash, PasswordVerifier};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use openssl::symm::encrypt;

use crate::http::{ApiState, ExtractorRejection, ResponseError, ResponseResult, UserTokenClaims};

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn post(
    State(api_state): State<ApiState>,
    WithRejection(Json(payload), _): WithRejection<Json<LoginPayload>, ExtractorRejection>,
) -> ResponseResult<impl IntoResponse> {
    let username = match encrypt(
        api_state.openssl_cipher,
        api_state.openssl_encryption_key.as_bytes(),
        None,
        payload.username.as_bytes(),
    ) {
        Ok(username) => hex::encode(username),
        Err(err) => {
            tracing::error!("Failed to encrypt username: {}", err);
            return Err(ResponseError {
                status: 500,
                message: Some(
                    "Failed to encrypt username! Contact a developer for further information"
                        .to_string(),
                ),
            });
        }
    };

    let user = match sqlx::query!(
        "SELECT id, password FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&api_state.database_pool)
    .await?
    {
        Some(user) => user,
        None => {
            return Err(ResponseError {
                status: 401,
                message: Some("Invalid username or password".to_string()),
            });
        }
    };

    let argon2 = argon2::Argon2::default();
    if argon2
        .verify_password(
            payload.password.as_bytes(),
            match &PasswordHash::new(&user.password) {
                Ok(hash) => hash,
                Err(err) => {
                    tracing::error!("Failed to verify password: {}", err);
                    return Err(ResponseError {
                        status: 500,
                        message: Some("Failed to verify password! Contact a developer for further information".to_string()),
                    });
                }
            },
        )
        .is_err()
    {
        return Err(ResponseError {
            status: 401,
            message: Some("Invalid credentials".to_string()),
        });
    };

    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &UserTokenClaims::new(user.id),
        &jsonwebtoken::EncodingKey::from_secret(api_state.session_secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(err) => {
            tracing::error!("Failed to generate token: {}", err);
            return Err(ResponseError {
                status: 500,
                message: Some(
                    "Failed to generate token! Contact a developer for further information"
                        .to_string(),
                ),
            });
        }
    };

    Ok(axum::Json(serde_json::json!({
        "success": true,
        "token": token,
    })))
}
