use crate::prelude::*;

pub async fn insert_vectors(
    pool: &PgPool,
    user_id: i32,
    vectors: Vec<f32>,
    metadata: Option<serde_json::Value>,
    database: String,
    content: Option<String>, 
) -> Result<(), ApiError> {
    let mut metadata_json = serde_json::json!({});
    if let Some(custom_metadata) = metadata {
        if let Some(obj) = custom_metadata.as_object() {
            for (key, value) in obj {
                metadata_json[key] = value.clone();
            }
        }
    }

    let tenant = format!("{}-{}", user_id, database);
    let vector_id = Uuid::new_v4().to_string();

    sqlx::query(
       "INSERT INTO vectors (tenant, vector_id, embedding, metadata, content) VALUES ($1, $2, $3, $4, $5)",
   )
   .bind(&tenant)
   .bind(&vector_id)
   .bind(&vectors)
   .bind(&metadata_json)
   .bind(&content) 
   .execute(pool)
   .await
   .map_err(|_| ApiError::DatabaseInsert)?;

    Ok(())
}
