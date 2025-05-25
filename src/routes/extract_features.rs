use crate::prelude::*;
use crate::structs::extract_features_struct::*;

pub async fn extract_features_handler(
    State(_state): State<AppState>,
    Json(payload): Json<ExtractFeaturesPayload>,
) -> Result<Json<ExtractFeaturesBody>, ExtractFeaturesError> {
    Ok(Json(ExtractFeaturesBody::new(payload.image)))
}
