use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub clip_model: Arc<clip::ClipModel>,
    pub clip_config: Arc<clip::ClipConfig>,
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
    pub tokenizer: Arc<Tokenizer>,
    pub pool: PgPool,
    pub google_client: AsyncClient,
    pub nsfw_model: Arc<Model>,
    pub task_queue: mpsc::UnboundedSender<BackgroundTask>,
    pub paddle: Paddle,
}

#[derive(Clone)]
pub struct WorkerState {
    //pub pool: PgPool,
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
}
