use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_vectors(
    user_id: i32,
    index: Arc<Mutex<Index>>,
    vectors: Vec<f32>,
    filename: String,
    database: String,
) -> Result<(), InsertImageError> {
    let mut fields = BTreeMap::new();
    fields.insert(
        "filename".to_string(),
        Value {
            kind: Some(Kind::StringValue(filename)),
        },
    );

    let metadata = Metadata { fields };
    let vectors = [Vector {
        id: Uuid::new_v4().to_string(),
        values: vectors,
        sparse_values: None,
        metadata: Some(metadata),
    }];
    let namespace = format!("{}-{}", user_id, database);

    let mut index = index.lock().await;
    index
        .upsert(&vectors, &namespace.into())
        .await
        .map_err(|_| InsertImageError::DatabaseInsert)?;

    Ok(())
}
