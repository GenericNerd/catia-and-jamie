use argon2::{password_hash::SaltString, PasswordHasher};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use openssl::symm::encrypt;

use crate::{
    http::{ApiState, ExtractorRejection, ResponseError, ResponseResult},
    snowflake::Snowflake,
};

#[derive(serde::Deserialize)]
pub struct SignupPayload {
    username: String,
    password: String,
}

lazy_static::lazy_static! {
    static ref SPECIAL_CHARS: regex::Regex = regex::Regex::new("[@_!#$%^&*()<>?/|}{~:]").unwrap();
}

pub async fn post(
    State(api_state): State<ApiState>,
    WithRejection(Json(payload), _): WithRejection<Json<SignupPayload>, ExtractorRejection>,
) -> ResponseResult<impl IntoResponse> {
    if payload.username.len() < 3 {
        return Err(ResponseError {
            status: 400,
            message: Some("Username must be at least 3 characters long".to_string()),
        });
    }
    if payload.username.len() > 32 {
        return Err(ResponseError {
            status: 400,
            message: Some("Username is too long".to_string()),
        });
    }

    if payload.password.len() < 8 {
        return Err(ResponseError {
            status: 400,
            message: Some("Password must be at least 8 characters long".to_string()),
        });
    }
    if payload.password.chars().all(char::is_lowercase) {
        return Err(ResponseError {
            status: 400,
            message: Some("Password must contain at least 1 uppercase character".to_string()),
        });
    }
    if payload.password.chars().all(char::is_alphabetic) {
        return Err(ResponseError {
            status: 400,
            message: Some("Password must contain at least 1 number".to_string()),
        });
    }
    if !SPECIAL_CHARS.is_match(&payload.password) {
        return Err(ResponseError {
            status: 400,
            message: Some("Password must contain at least 1 special character".to_string()),
        });
    }

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
                    "Failed to encrypt the username field! Contact a developer for further information"
                .to_string()),
            });
        }
    };

    if sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_optional(&api_state.database_pool)
        .await?
        .is_some()
    {
        return Err(ResponseError {
            status: 409,
            message: Some("Username already in use".to_string()),
        });
    }

    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = argon2::Argon2::default();
    let hashed_password = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(err) => {
            tracing::error!("Failed to hash password: {}", err);
            return Err(ResponseError {
                status: 500,
                message: Some(
                    "Failed to hash the password field! Contact a developer for further information"
                .to_string()),
            });
        }
    };

    let user_id = Snowflake::new();

    sqlx::query!(
        "INSERT INTO users (id, username, password) VALUES ($1, $2, $3)",
        user_id.snowflake as i64,
        hex::encode(username),
        hashed_password.to_string()
    )
    .execute(&api_state.database_pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        axum::Json(serde_json::json!({
            "success": true,
        })),
    ))
}
