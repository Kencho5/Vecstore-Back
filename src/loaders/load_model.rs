use crate::prelude::*;

//pub fn load_clip_model() -> Result<(clip::ClipModel, clip::ClipConfig), Box<dyn std::error::Error>>
//{
//    println!("Loading model...");
//    let device = Device::Cpu;
//
//    let api = hf_hub::api::sync::Api::new()?;
//    let repo = api.repo(hf_hub::Repo::with_revision(
//        "openai/clip-vit-base-patch32".to_string(),
//        hf_hub::RepoType::Model,
//        "refs/pr/15".to_string(),
//    ));
//    let model_path = repo.get("model.safetensors")?;
//
//    let config = clip::ClipConfig::vit_base_patch32();
//    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
//    let model = clip::ClipModel::new(vb, &config)?;
//
//    println!("Model loaded");
//
//    Ok((model, config))
//}
//
//pub fn load_nsfw_model() -> Result<Model, Box<dyn std::error::Error>> {
//    let device = Device::Cpu;
//    let api = hf_hub::api::sync::Api::new()?;
//    let repo = api.repo(hf_hub::Repo::new(
//        "Falconsai/nsfw_image_detection".to_string(),
//        hf_hub::RepoType::Model,
//    ));
//
//    let model_path = repo.get("model.safetensors")?;
//    let config_path = repo.get("config.json")?;
//
//    let config: Config = serde_json::from_reader(std::fs::File::open(&config_path)?)?;
//    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
//
//    Ok(Model::new(&config, 2, vb)?)
//}

pub fn load_clip_model() -> Result<(clip::ClipModel, clip::ClipConfig), Box<dyn std::error::Error>>
{
    println!("Loading model...");

    let config = clip::ClipConfig::vit_base_patch32();

    // Create an empty/default model without loading weights
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = clip::ClipModel::new(vb, &config)?;

    println!("Model loaded");
    Ok((model, config))
}

pub fn load_nsfw_model() -> Result<Model, Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    // Create a default config using the vit_base_patch16_224 preset
    let config = Config::vit_base_patch16_224();

    // Create empty variable builder
    let vb = VarBuilder::zeros(DType::F32, &device);

    Ok(Model::new(&config, 2, vb)?)
}
