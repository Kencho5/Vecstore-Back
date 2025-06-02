use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub clip_model: mobileclip::MobileClipModel,
    pub clip_config: mobileclip::MobileClipConfig,
    pub pinecone: PineconeClient,
    pub tokenizer: Tokenizer,
    pub pool: PgPool,
    pub google_client: AsyncClient,
    pub nsfw_model: Model,
}
