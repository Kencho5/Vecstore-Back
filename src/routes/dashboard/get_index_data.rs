use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn index_data_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<NamespaceStats>, DashboardError> {
    let mut index = state
        .pinecone
        .index(&env::var("PINECONE_INDEX").expect("Pinecone index not found"))
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let stats = index
        .describe_index_stats(None)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let vector_count = stats
        .namespaces
        .get(&claims.email)
        .map(|ns| ns.vector_count)
        .unwrap_or(0);

    Ok(Json(NamespaceStats {
        record_count: vector_count,
        size: format!("{:.2}MB", (vector_count * 512 * 8) as f64 / 1024.0 / 1024.0),
    }))
}
