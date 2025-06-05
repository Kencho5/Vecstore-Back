use crate::prelude::*;

#[derive(Debug)]
pub enum BackgroundTask {
    InsertVectors {
        user_id: i32,
        vectors: Vec<f32>,
        filename: String,
        database: String,
    },
    IncrementRequest {
        database: String,
    },
}

pub async fn process_task_queue(
    mut receiver: mpsc::UnboundedReceiver<BackgroundTask>,
    state: WorkerState,
) {
    while let Some(task) = receiver.recv().await {
        match task {
            BackgroundTask::InsertVectors {
                user_id,
                vectors,
                filename,
                database,
            } => {
                if let Err(e) =
                    insert_vectors(user_id, &state.pinecone, vectors, filename, database).await
                {
                    eprintln!("Failed to insert vectors: {:?}", e);
                }
            }
            BackgroundTask::IncrementRequest { database } => {
                if let Err(e) = increment_req(&state.pool, database).await {
                    eprintln!("Failed to increment requests: {:?}", e);
                }
            }
        }
    }
}
