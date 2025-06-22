use crate::prelude::*;
use axum::middleware::Next;

pub async fn api_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(api_key);

    Ok(next.run(req).await)
}
