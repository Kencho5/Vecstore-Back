use crate::prelude::*;

pub async fn load_bedrock_client() -> BedrockClient {
    let aws_access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found");
    let aws_secret_key =
        env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found");
    let aws_region = "eu-central-1".to_string();

    let credentials = Credentials::new(
        aws_access_key,
        aws_secret_key,
        None,
        None,
        "env-credentials",
    );

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(aws_region))
        .credentials_provider(credentials)
        .load()
        .await;

    BedrockClient::new(&aws_config)
}
