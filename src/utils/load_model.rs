use std::path::{Path, PathBuf};

pub fn load_model() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let local_path = "models/clip-vit-base-patch32/model.safetensors";

    if Path::new(local_path).exists() {
        println!("Using local model at: {}", local_path);
        return Ok(PathBuf::from(local_path));
    }

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
    Ok(PathBuf::from(local_path))
}
