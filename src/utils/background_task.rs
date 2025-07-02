use crate::prelude::*;
use futures::stream::{FuturesUnordered, StreamExt};

#[derive(Debug)]
pub enum BackgroundTask {
    InsertImageVectors {
        user_id: i32,
        image_data: Vec<u8>,
        metadata: Option<String>,
        database: String,
        region: String,
    },
    InsertTextVectors {
        user_id: i32,
        text: String,
        metadata: Option<String>,
        database: String,
        region: String,
    },
    SaveUsageLogs {
        user_id: i32,
    },
}

pub async fn process_task_queue(
    mut receiver: mpsc::UnboundedReceiver<BackgroundTask>,
    state: WorkerState,
) {
    let mut futures = FuturesUnordered::new();
    let max_concurrent = 50;

    loop {
        tokio::select! {
            task = receiver.recv() => {
                match task {
                    Some(task) => {
                        let state_clone = state.clone();
                        let future = tokio::spawn(async move {
                            process_single_task(task, state_clone).await
                        });
                        futures.push(future);

                        if futures.len() >= max_concurrent {
                            if let Some(result) = futures.next().await {
                                if let Err(e) = result {
                                    eprintln!("Task failed: {:?}", e);
                                }
                            }
                        }
                    }
                    None => break,
                }
            }

            result = futures.next(), if !futures.is_empty() => {
                if let Some(task_result) = result {
                    if let Err(e) = task_result {
                        eprintln!("Task failed: {:?}", e);
                    }
                }
            }
        }
    }

    while let Some(result) = futures.next().await {
        if let Err(e) = result {
            eprintln!("Task failed: {:?}", e);
        }
    }
}

async fn process_single_task(task: BackgroundTask, state: WorkerState) {
    match task {
        BackgroundTask::InsertImageVectors {
            user_id,
            image_data,
            metadata,
            database,
            region,
        } => {
            if let Some(pool) = state.neon_pools.get_pool_by_region(&region) {
                let vectors = extract_image_features(&state.bedrock_client, image_data)
                    .await
                    .unwrap();

                if let Err(e) = insert_vectors(pool, user_id, vectors, metadata, database).await {
                    eprintln!("Failed to insert vectors: {:?}", e);
                }
            } else {
                eprintln!("Failed to get index for region: {}", region);
            }
        }
        BackgroundTask::InsertTextVectors {
            user_id,
            text,
            metadata,
            database,
            region,
        } => {
            if let Some(pool) = state.neon_pools.get_pool_by_region(&region) {
                let vectors = extract_text_features(&state.bedrock_client, text)
                    .await
                    .unwrap();

                if let Err(e) = insert_vectors(pool, user_id, vectors, metadata, database).await {
                    eprintln!("Failed to insert vectors: {:?}", e);
                }
            } else {
                eprintln!("Failed to get index for region: {}", region);
            }
        }
        BackgroundTask::SaveUsageLogs { user_id } => {
            if let Err(e) = save_usage_logs(state.pool.clone(), user_id).await {
                eprintln!("Failed to save logs: {:?}", e);
            }
        }
    }
}
