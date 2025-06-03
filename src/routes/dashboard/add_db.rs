use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, AddDbError> {
    payload.validate().map_err(|_| AddDbError::MissingDbData)?;

    let user = validate_token(headers)
        .await
        .map_err(|_| AddDbError::Unauthorized)?;
    println!("{:?}", user.email);

    sqlx::query("INSERT INTO databases(name, type, region, owner_email) VALUES($1, $2, $3, $4)")
        .bind(&payload.name)
        .bind(&payload.db_type)
        .bind(&payload.region)
        .bind(&user.email)
        .execute(&state.pool)
        .await
        .map_err(|_| AddDbError::DatabaseExists)?;

    Ok(StatusCode::OK)
}
