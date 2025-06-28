use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
    pub pool: PgPool,
    pub google_client: AsyncClient,
    pub task_queue: mpsc::UnboundedSender<BackgroundTask>,
    pub paddle: Paddle,
    pub bedrock_client: BedrockClient,
    pub ses_client: aws_sdk_sesv2::Client,
}

#[derive(Clone)]
pub struct WorkerState {
    pub pool: PgPool,
    pub pinecone_indexes: Arc<Mutex<PineconeIndexes>>,
    pub bedrock_client: BedrockClient,
}
