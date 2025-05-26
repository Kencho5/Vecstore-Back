use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub model: clip::ClipModel,
    pub clip_config: clip::ClipConfig,
    pub qdrant_client: Arc<Qdrant>,
}
