use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub clip_model: clip::ClipModel,
    pub clip_config: clip::ClipConfig,
    pub pinecone: PineconeClient,
    pub tokenizer: Tokenizer,
    pub pool: PgPool,
    pub google_client: AsyncClient,
    pub nsfw_model: Model,
}
