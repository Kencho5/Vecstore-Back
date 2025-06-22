use crate::prelude::*;
use crate::structs::search_struct::*;

pub async fn search_vectors(
    state: &AppState,
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
) -> Result<SearchResponse, SearchImageError> {
    let indexes = state.pinecone_indexes.lock().await;
    let region = get_db_region(&state.pool, &database, &user_id).await?;
    let index = indexes.get_index_by_region(&region).unwrap();
    let mut index = index.lock().await;

    let response: QueryResponse = index
        .query_by_value(
            vectors,
            None,
            3,
            &Namespace::from(format!("{}-{}", user_id, database)),
            None,
            None,
            Some(true),
        )
        .await
        .map_err(|_| SearchImageError::Unforseen)?;

    // Convert QueryResponse to SearchResponse
    let search_response = SearchResponse {
        matches: response
            .matches
            .into_iter()
            .map(|m| SearchMatch {
                score: format!("{:.2}%", m.score * 100.0),
                filename: m.metadata.and_then(|metadata| {
                    metadata.fields.get("filename").and_then(|value| {
                        if let Some(Kind::StringValue(s)) = &value.kind {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                }),
            })
            .collect(),
    };

    Ok(search_response)
}
