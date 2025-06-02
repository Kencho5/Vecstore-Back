use crate::prelude::*;

pub fn load_clip_model(
) -> Result<(mobileclip::MobileClipModel, mobileclip::MobileClipConfig), Box<dyn std::error::Error>>
{
    let device = Device::Cpu;
    let api = Api::new()?;
    let repo = api.model("apple/MobileCLIP-S1-OpenCLIP".to_string());

    let model_path = repo.get("open_clip_model.safetensors")?;
    let config = mobileclip::MobileClipConfig::s1();

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
    let model = mobileclip::MobileClipModel::new(vb, &config)?;
    Ok((model, config))
}

pub fn load_nsfw_model() -> Result<Model, Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    let api = hf_hub::api::sync::Api::new()?;
    let repo = api.repo(hf_hub::Repo::new(
        "Falconsai/nsfw_image_detection".to_string(),
        hf_hub::RepoType::Model,
    ));

    let model_path = repo.get("model.safetensors")?;
    let config_path = repo.get("config.json")?;

    let config: Config = serde_json::from_reader(std::fs::File::open(&config_path)?)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };

    Ok(Model::new(&config, 2, vb)?)
}
