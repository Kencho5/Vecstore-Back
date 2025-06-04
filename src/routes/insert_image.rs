use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_image_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<InsertImagePayload>,
) -> Result<Json<InsertImageBody>, InsertImageError> {
    payload
        .validate()
        .map_err(|_| InsertImageError::MissingData)?;

    let total_start = Instant::now();

    let image_vectors = extract_image_features(&state, payload.image).await?;

    let insert_task = BackgroundTask::InsertVectors {
        user_id: claims.user_id,
        vectors: image_vectors,
        filename: payload.filename,
        database: payload.database.clone(),
    };
    let increment_task = BackgroundTask::IncrementRequest {
        database: payload.database,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    if state.task_queue.send(increment_task).is_err() {
        eprintln!("Failed to send increment_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total CLIP handler time: {}ms", total_time_ms);

    Ok(Json(InsertImageBody::new(total_time_ms)))
}

async fn extract_image_features(
    state: &AppState,
    image: String,
) -> Result<Vec<f32>, InsertImageError> {
    let start_time = Instant::now();

    let image = load_image::load_image(image, state.clip_config.image_size)
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let image_features = state
        .clip_model
        .get_image_features(&image)
        .map_err(|_| InsertImageError::ModelInference)?;

    let image_vector = image_features
        .flatten_all()
        .map_err(|_| InsertImageError::ImageProcessing)?
        .to_vec1::<f32>()
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let total_processing_time_ms = start_time.elapsed().as_millis();
    println!(
        "Total extract_image_features took: {}ms",
        total_processing_time_ms
    );

    Ok(image_vector)
}

pub async fn insert_vectors(
    pinecone: &PineconeClient,
    vectors: Vec<f32>,
    filename: String,
    user_id: i32,
    database: String,
) -> Result<(), InsertImageError> {
    let mut index = pinecone
        .index(&env::var("PINECONE_INDEX").expect("Pinecone index not found"))
        .await
        .map_err(|_| InsertImageError::DatabaseConnection)?;

    let mut fields = BTreeMap::new();
    let filename_value = Value {
        kind: Some(Kind::StringValue(filename)),
    };
    fields.insert("filename".to_string(), filename_value);

    let metadata = Metadata { fields };

    let vectors = [Vector {
        id: Uuid::new_v4().to_string(),
        values: vectors,
        sparse_values: None,
        metadata: Some(metadata),
    }];

    let namespace = format!("{}-{}", user_id, database);

    index
        .upsert(&vectors, &namespace.into())
        .await
        .map_err(|_| InsertImageError::DatabaseInsert)?;

    Ok(())
}
