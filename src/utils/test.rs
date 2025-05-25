use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::clip;
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use qdrant_client::{Payload, Qdrant};
use std::time::Instant;

fn load_image(path: &str, image_size: usize) -> Result<Tensor> {
    let img = image::ImageReader::open(path)?.decode()?;
    let img = img.resize_to_fill(
        image_size as u32,
        image_size as u32,
        image::imageops::FilterType::Triangle,
    );
    let img = img.to_rgb8();
    let img = img.into_raw();
    let img = Tensor::from_vec(img, (image_size, image_size, 3), &Device::Cpu)?
        .permute((2, 0, 1))?
        .to_dtype(DType::F32)?
        .affine(2. / 255., -1.)?;
    Ok(img)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Qdrant::from_url(
        "https://155e571e-e499-417d-9612-77d4fa1cbd54.europe-west3-0.gcp.cloud.qdrant.io:6334",
    )
        .api_key("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2Nlc3MiOiJtIn0.7sET3eaLqqRhhffAzGP-xKxvJUsq1E2kpXUM-Mz0OT4")
        .skip_compatibility_check()
        .build()?;

    let total_start = Instant::now();

    let device = Device::Cpu;
    let config = clip::ClipConfig::vit_base_patch32();

    // Download model
    println!("Downloading model...");
    let model_start = Instant::now();
    let api = hf_hub::api::sync::Api::new()?;
    let repo = api.repo(hf_hub::Repo::with_revision(
        "openai/clip-vit-base-patch32".to_string(),
        hf_hub::RepoType::Model,
        "refs/pr/15".to_string(),
    ));
    let model_file = repo.get("model.safetensors")?;

    // Load model
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };
    let model = clip::ClipModel::new(vb, &config)?;
    println!(
        "Model loaded in: {:.2}s",
        model_start.elapsed().as_secs_f64()
    );

    // Load image
    let image_path = "test.jpg";
    let load_start = Instant::now();
    let image = load_image(image_path, config.image_size)?;
    let images = image.unsqueeze(0)?;
    println!("Image loaded in: {:.2}ms", load_start.elapsed().as_millis());

    // Get image features (the actual transformation)
    let transform_start = Instant::now();
    let image_features = model.get_image_features(&images)?;
    let image_vector = image_features.flatten_all()?.to_vec1::<f32>()?;
    let transform_time = transform_start.elapsed();

    println!(
        "Image transformed to vector in: {:.2}ms",
        transform_time.as_millis()
    );
    println!("Total time: {:.2}s", total_start.elapsed().as_secs_f64());
    println!("Image vector dimensions: {}", image_vector.len());
    println!("First 10 values: {:?}", &image_vector[0..10]);

    let payload: Payload = serde_json::json!(
        {
            "filename": "test.jpg",
        }
    )
    .try_into()
    .unwrap();
    let points = vec![PointStruct::new(12343, image_vector, payload)];

    client
        .upsert_points(UpsertPointsBuilder::new("images", points))
        .await?;

    Ok(())
}
