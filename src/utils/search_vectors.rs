use crate::prelude::*;

pub async fn search_vectors_with_region(
    state: &AppState,
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
    region: &String,
    metadata_filter: Option<String>,
) -> Result<SearchResults, ApiError> {
    let index = state.pinecone_indexes.get_index_by_region(region).unwrap();
    let mut index = index.lock().await;

    search_vectors_impl(vectors, user_id, database, &mut index, metadata_filter).await
}

async fn search_vectors_impl(
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
    index: &mut Index,
    metadata_filter: Option<String>,
) -> Result<SearchResults, ApiError> {
    // Parse metadata filter if provided
    let filter = if let Some(metadata_str) = metadata_filter {
        match serde_json::from_str::<serde_json::Value>(&metadata_str) {
            Ok(json_value) => {
                if let Some(obj) = json_value.as_object() {
                    let mut fields = BTreeMap::new();
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
                    Some(Metadata { fields })
                } else {
                    return Err(ApiError::InvalidMetadata);
                }
            }
            Err(_) => return Err(ApiError::InvalidMetadata),
        }
    } else {
        None
    };

    let response: QueryResponse = index
        .query_by_value(
            vectors,
            None,
            3,
            &Namespace::from(format!("{}-{}", user_id, database)),
            filter,
            None,
            Some(true),
        )
        .await
        .map_err(|_| ApiError::Unforseen)?;

    // Convert QueryResponse to SearchResults
    let search_response = SearchResults {
        matches: response
            .matches
            .into_iter()
            .map(|m| {
                let metadata = m.metadata.map(|md| {
                    md.fields
                        .into_iter()
                        .filter_map(|(k, v)| {
                            if let Some(Kind::StringValue(s)) = v.kind {
                                Some((k, s))
                            } else {
                                None
                            }
                        })
                        .collect()
                });
                let id = m.id;

                SearchMatch {
                    id,
                    score: format!("{:.2}%", m.score * 100.0),
                    metadata,
                }
            })
            .collect(),
    };

    Ok(search_response)
}
