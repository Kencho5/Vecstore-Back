use crate::prelude::*;

pub async fn insert_vectors(
    pool: &PgPool,
    user_id: i32,
    vectors: Vec<f32>,
    metadata: Option<String>,
    database: String,
) -> Result<(), ApiError> {
    let mut metadata_json = serde_json::json!({});

    if let Some(custom_metadata) = metadata {
        match serde_json::from_str::<serde_json::Value>(&custom_metadata) {
            Ok(json_value) => {
                if let Some(obj) = json_value.as_object() {
                    for (key, value) in obj {
                        metadata_json[key] = value.clone();
                    }
                }
            }
            Err(_) => return Err(ApiError::InvalidMetadata),
        }
    }

    let tenant = format!("{}-{}", user_id, database);
    let vector_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO vectors (tenant, vector_id, embedding, metadata) VALUES ($1, $2, $3, $4)",
    )
    .bind(&tenant)
    .bind(&vector_id)
    .bind(&vectors)
    .bind(&metadata_json)
    .execute(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    Ok(())
}
