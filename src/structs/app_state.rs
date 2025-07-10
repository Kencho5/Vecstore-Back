use crate::prelude::*;
use mini_moka::sync::Cache;

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
    pub user_cache: Cache<String, UserCacheResult>,
}

#[derive(Clone)]
pub struct WorkerState {
    pub pool: PgPool,
    pub neon_pools: NeonPools,
    pub bedrock_client: BedrockClient,
}
