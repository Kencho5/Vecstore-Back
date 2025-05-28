use crate::prelude::*;
use anyhow::Error as E;

pub fn get_tokenizer(tokenizer: Option<String>) -> anyhow::Result<Tokenizer> {
    let tokenizer = match tokenizer {
        None => {
            let api = hf_hub::api::sync::Api::new()?;
            let api = api.repo(hf_hub::Repo::with_revision(
                "openai/clip-vit-base-patch32".to_string(),
                hf_hub::RepoType::Model,
                "refs/pr/15".to_string(),
            ));
            api.get("tokenizer.json")?
        }
        Some(file) => file.into(),
    };
    Tokenizer::from_file(tokenizer).map_err(E::msg)
}
