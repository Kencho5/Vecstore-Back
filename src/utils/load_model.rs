use crate::prelude::*;

pub fn load_model() -> Result<(clip::ClipModel, clip::ClipConfig), Box<dyn std::error::Error>> {
    println!("Loading model...");
    let device = Device::Cpu;

    let api = hf_hub::api::sync::Api::new()?;
    let repo = api.repo(hf_hub::Repo::with_revision(
        "openai/clip-vit-base-patch32".to_string(),
        hf_hub::RepoType::Model,
        "refs/pr/15".to_string(),
    ));
    let model_path = repo.get("model.safetensors")?;

    let config = clip::ClipConfig::vit_base_patch32();
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
    let model = clip::ClipModel::new(vb, &config)?;

    println!("Model loaded");

    Ok((model, config))
}
