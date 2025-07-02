use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub neon_pools: NeonPools,
    pub google_client: AsyncClient,
    pub task_queue: mpsc::UnboundedSender<BackgroundTask>,
    pub paddle: Paddle,
    pub bedrock_client: BedrockClient,
    pub ses_client: SesClient,
    pub rekognition_client: RekognitionClient,
}

#[derive(Clone)]
pub struct WorkerState {
    pub pool: PgPool,
    pub neon_pools: NeonPools,
    pub bedrock_client: BedrockClient,
}
