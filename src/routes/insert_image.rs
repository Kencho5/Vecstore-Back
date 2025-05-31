use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_image_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertImagePayload>,
) -> Result<Json<InsertImageBody>, InsertImageError> {
    let total_start = Instant::now();
    println!("CLIP Handler started");

    let image_vectors = extract_image_features(&state, payload.image).await?;

    insert_vectors(state.pinecone, image_vectors, payload.filename).await?;

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

async fn insert_vectors(
    pinecone: PineconeClient,
    vectors: Vec<f32>,
    filename: String,
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

    index
        .upsert(&vectors, &"kencho".into())
        .await
        .map_err(|_| InsertImageError::DatabaseInsert)?;

    Ok(())
}
