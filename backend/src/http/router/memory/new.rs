use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    http::{ApiState, ResponseResult},
    snowflake::Snowflake,
};

pub async fn post(
    State(state): State<ApiState>,
    mut multipart: Multipart,
) -> ResponseResult<impl IntoResponse> {
    let mut memory_ids = vec![];
    let mut table = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();
        match field_name {
            "memory" => {
                let snowflake = Snowflake::new();
                let data = field.bytes().await.unwrap();
                let mut file = File::create(format!("memories/{}.png", snowflake.snowflake))
                    .await
                    .unwrap();
                file.write(&data).await.unwrap();
                memory_ids.push(snowflake.snowflake);
            }
            "table" => {
                table = Some(field.text().await.unwrap());
            }
            _ => {}
        }
    }

    for memory_id in memory_ids {
        sqlx::query!(
            "INSERT INTO memories (memory_id, table_name, url) VALUES ($1, $2, $3)",
            memory_id as i64,
            match table {
                Some(ref table) => table.clone(),
                None => "Unknown".to_string(),
            },
            format!("/memories/{}.png", memory_id)
        )
        .execute(&state.database_pool)
        .await?;
    }
    // for image in payload.images {
    //     let image = match URL_SAFE.decode(image.as_bytes()) {
    //         Ok(decode) => Reader::new(Cursor::new(decode))
    //             .with_guessed_format()?
    //             .decode()?,
    //         Err(err) => {
    //             return Err(ResponseError {
    //                 status: 400,
    //                 message: Some(err.to_string()),
    //             })
    //         }
    //     };

    //     image.save(format!("memories/{}.png", snowflake.snowflake))?;
    //     sqlx::query!(
    //         "INSERT INTO memories (memory_id, table_name, url) VALUES ($1, $2, $3)",
    //         snowflake.snowflake as i64,
    //         payload.table,
    //         format!("/memories/{}.png", snowflake.snowflake)
    //     )
    //     .execute(&state.database_pool)
    //     .await?;
    // }

    Ok(())
}
