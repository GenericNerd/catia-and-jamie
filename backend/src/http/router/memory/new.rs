use std::io::Cursor;

use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use base64::{engine::general_purpose::URL_SAFE, Engine};
use image::io::Reader;

use crate::{
    http::{ApiState, ExtractorRejection, ResponseError, ResponseResult},
    snowflake::Snowflake,
};

#[derive(serde::Deserialize)]
pub struct NewMemory {
    table: String,
    images: Vec<String>,
}

pub async fn post(
    State(state): State<ApiState>,
    WithRejection(Json(payload), _): WithRejection<Json<NewMemory>, ExtractorRejection>,
) -> ResponseResult<impl IntoResponse> {
    for image in payload.images {
        let image = match URL_SAFE.decode(image.as_bytes()) {
            Ok(decode) => Reader::new(Cursor::new(decode))
                .with_guessed_format()?
                .decode()?,
            Err(err) => {
                return Err(ResponseError {
                    status: 400,
                    message: Some(err.to_string()),
                })
            }
        };

        let snowflake = Snowflake::new();
        image.save(format!("memories/{}.png", snowflake.snowflake))?;
        sqlx::query!(
            "INSERT INTO memories (memory_id, table_name, url) VALUES ($1, $2, $3)",
            snowflake.snowflake as i64,
            payload.table,
            format!("/memories/{}.png", snowflake.snowflake)
        )
        .execute(&state.database_pool)
        .await?;
    }

    Ok(())
}
