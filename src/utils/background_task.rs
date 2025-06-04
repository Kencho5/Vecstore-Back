use crate::prelude::*;

#[derive(Debug)]
pub enum BackgroundTask {
    InsertVectors {
        user_id: i32,
        vectors: Vec<f32>,
        filename: String,
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
                    insert_vectors(&state.pinecone, vectors, filename, user_id, database).await
                {
                    eprintln!("Failed to insert vectors: {:?}", e);
                }
            }
        }
    }
}
