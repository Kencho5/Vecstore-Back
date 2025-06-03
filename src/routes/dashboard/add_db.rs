use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, AddDbError> {
    println!("{:?}", payload.name);
    Ok(StatusCode::OK)
}
