use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, AddDbError> {
    payload.validate().map_err(|_| AddDbError::MissingDbData)?;

    println!("{:?}", payload.name);
    Ok(StatusCode::OK)
}
