use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub clip_model: clip::ClipModel,
    pub clip_config: clip::ClipConfig,
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
    pub tokenizer: Tokenizer,
    pub pool: PgPool,
    pub google_client: AsyncClient,
    pub nsfw_model: Model,
    pub task_queue: mpsc::UnboundedSender<BackgroundTask>,
    pub paddle: Paddle,
}

#[derive(Clone)]
pub struct WorkerState {
    //pub pool: PgPool,
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
}
