use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn index_data_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<GetDbPayload>,
) -> Result<Json<NamespaceStats>, DashboardError> {
    let indexes = state.pinecone_indexes.lock().await;
    let mut index = indexes.image_us_east.lock().await;

    let stats = index
        .describe_index_stats(None)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let vector_count = stats
        .namespaces
        .get(&format!("{}-{}", claims.user_id, payload.name))
        .map(|ns| ns.vector_count)
        .unwrap_or(0);

    Ok(Json(NamespaceStats {
        record_count: vector_count,
        //size: format!("{:.2}MB", (vector_count * 512 * 8) as f64 / 1024.0 / 1024.0),
    }))
}
