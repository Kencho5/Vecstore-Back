use crate::prelude::*;

pub async fn create_token(
    user_id: i32,
    email: String,
    name: String,
    plan_names: Vec<String>,
) -> Result<String, AuthError> {
    let exp = Utc::now() + Duration::days(7);

    let claims = Claims {
        user_id,
        email,
        name,
        exp: exp.timestamp_millis(),
        plan_names,
    };

    let key = env::var("SECRET_KEY").expect("Secret key not set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)
}

pub async fn validate_token(headers: HeaderMap) -> Result<Claims, AuthError> {
    let token = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if token.is_none() {
        return Err(AuthError::InvalidToken);
    }

    let key = env::var("SECRET_KEY").expect("Secret key not set");
    let token_data = decode::<Claims>(
        &token.unwrap(),
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    let expiry = DateTime::from_timestamp_millis(token_data.claims.exp).unwrap();
    if expiry <= Utc::now() {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}
