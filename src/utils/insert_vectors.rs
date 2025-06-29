use crate::prelude::*;

pub async fn insert_vectors(
    user_id: i32,
    mut index: Index,
    vectors: Vec<f32>,
    filename: Option<String>,
    metadata: Option<String>,
    database: String,
) -> Result<(), ApiError> {
    let mut fields = BTreeMap::new();

    if let Some(filename_value) = filename {
        fields.insert(
            "filename".to_string(),
            Value {
                kind: Some(Kind::StringValue(filename_value)),
            },
        );
    }

    if let Some(custom_metadata) = metadata {
        match serde_json::from_str::<serde_json::Value>(&custom_metadata) {
            Ok(json_value) => {
                if let Some(obj) = json_value.as_object() {
                    for (key, value) in obj {
                        let pinecone_value = match value {
                            serde_json::Value::String(s) => Value {
                                kind: Some(Kind::StringValue(s.clone())),
                            },
                            serde_json::Value::Number(n) => {
                                if let Some(f) = n.as_f64() {
                                    Value {
                                        kind: Some(Kind::NumberValue(f)),
                                    }
                                } else {
                                    continue;
                                }
                            }
                            serde_json::Value::Bool(b) => Value {
                                kind: Some(Kind::BoolValue(*b)),
                            },
                            _ => continue,
                        };
                        fields.insert(key.clone(), pinecone_value);
                    }
                }
            }
            Err(_) => return Err(ApiError::InvalidMetadata),
        }
    }

    let metadata = Metadata { fields };
    let vectors = [Vector {
        id: Uuid::new_v4().to_string(),
        values: vectors,
        sparse_values: None,
        metadata: Some(metadata),
    }];
    let namespace = format!("{}-{}", user_id, database);

    index
        .upsert(&vectors, &namespace.into())
        .await
        .map_err(|_| ApiError::DatabaseInsert)?;

    Ok(())
}
