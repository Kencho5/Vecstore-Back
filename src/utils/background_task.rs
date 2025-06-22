use crate::prelude::*;
use futures::stream::{FuturesUnordered, StreamExt};

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
        BackgroundTask::InsertVectors {
            user_id,
            vectors,
            filename,
            database,
        } => {
            let indexes = state.pinecone_indexes.lock().await;

            let region = increment_req(&state.pool, &database, user_id)
                .await
                .unwrap();

            let index = match region.as_str() {
                "us-east-1-image" => indexes.image_us_east.clone(),
                "us-east-1-text" => indexes.text_us_east.clone(),
                _ => {
                    eprintln!("Unknown database/region: {}", database);
                    return;
                }
            };

            if let Err(e) = insert_vectors(user_id, index, vectors, filename, database).await {
                eprintln!("Failed to insert vectors: {:?}", e);
            }
        }
        BackgroundTask::IncrementRequest { database, user_id } => {
            if let Err(e) = increment_req(&state.pool, &database, user_id).await {
                eprintln!("Failed to increment requests: {:?}", e);
            }
        }
    }
}
