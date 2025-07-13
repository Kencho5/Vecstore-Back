use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_dbs_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Database>>, DashboardError> {
    let mut dbs = sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_id = $1")
        .bind(&claims.user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let mut tasks = Vec::new();
    
    for db in &dbs {
        let neon_pool = state
            .neon_pools
            .get_pool_by_region(&db.region)
            .ok_or(DashboardError::Unforseen)?;
        
        let tenant = format!("{}-{}", claims.user_id, db.name);
        let task = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM vectors WHERE tenant = $1")
            .bind(tenant)
            .fetch_one(neon_pool);
        tasks.push(task);
    }
    
    let results = futures::future::try_join_all(tasks)
        .await
        .map_err(|_| DashboardError::Unforseen)?;
    
    for (db, result) in dbs.iter_mut().zip(results.iter()) {
        db.record_count = Some(result.0);
    }

    Ok(Json(dbs))
}

pub async fn get_db_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<GetDbPayload>,
) -> Result<Json<Database>, DashboardError> {
    let mut result =
        sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_id = $1 AND name = $2")
            .bind(&claims.user_id)
            .bind(&payload.name)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| DashboardError::Unforseen)?;

    let neon_pool = state
        .neon_pools
        .get_pool_by_region(&result.region)
        .ok_or(DashboardError::Unforseen)?;

    let tenant = format!("{}-{}", claims.user_id, result.name);
    let record_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vectors WHERE tenant = $1")
        .bind(&tenant)
        .fetch_one(neon_pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    result.record_count = Some(record_count.0);
    Ok(Json(result))
}

pub async fn get_db_documents_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<GetDbDocumentsPayload>,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
    let result =
        sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_id = $1 AND name = $2")
            .bind(&claims.user_id)
            .bind(&payload.name)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| DashboardError::Unforseen)?;

    let neon_pool = state
        .neon_pools
        .get_pool_by_region(&result.region)
        .ok_or(DashboardError::Unforseen)?;

    let tenant = format!("{}-{}", claims.user_id, result.name);
    let documents = sqlx::query_as::<_, DatabaseDocument>(
        "SELECT vector_id, content, metadata, created_at FROM vectors WHERE tenant = $1 ORDER BY created_at DESC LIMIT 5 OFFSET $2",
    )
    .bind(&tenant)
    .bind(&payload.page)
    .fetch_all(neon_pool)
    .await
    .map_err(|e|{dbg!(e); DashboardError::Unforseen})?;

    Ok(Json(documents))
}
