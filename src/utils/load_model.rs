use crate::prelude::*;
use std::path::{Path, PathBuf};

pub fn load_model() -> Result<(clip::ClipModel, clip::ClipConfig), Box<dyn std::error::Error>> {
    let local_path = "models/clip-vit-base-patch32/model.safetensors";

    // Download model if it doesn't exist locally
    if !Path::new(local_path).exists() {
        println!("Model not found locally, downloading...");
        let api = hf_hub::api::sync::Api::new()?;
        let repo = api.repo(hf_hub::Repo::with_revision(
            "openai/clip-vit-base-patch32".to_string(),
            hf_hub::RepoType::Model,
            "refs/pr/15".to_string(),
        ));
        let cached_file = repo.get("model.safetensors")?;
        std::fs::create_dir_all("models/clip-vit-base-patch32")?;
        std::fs::copy(&cached_file, local_path)?;
        println!("Model downloaded and saved to: {}", local_path);
    } else {
        println!("Using local model at: {}", local_path);
    }

    // Load the model
    println!("Loading model...");
    let model_load_start = Instant::now();

    let device = Device::Cpu;
    let clip_config = clip::ClipConfig::vit_base_patch32();
    let model_path = PathBuf::from(local_path);
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[&model_path], DType::F32, &device)? };
    let model = clip::ClipModel::new(vb, &clip_config)?;

    let model_load_time = model_load_start.elapsed().as_millis();
    println!("Model loaded in: {}ms", model_load_time);

    Ok((model, clip_config))
}
