use crate::{errors::AppError, services::auth::verify_access_token};
use axum::{
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                Some(auth_value.to_owned())
            }
        });

    let token = token.ok_or(AppError::TokenNotFound)?;

    let verified_token = verify_access_token(&token).await?;
    let token_claims = verified_token
        .payload_claims()
        .ok_or(AppError::TokenPayloadError)?;
    let token_subject = token_claims
        .get_claim("sub")
        .ok_or(AppError::TokenPayloadError)?;
    let token_subject_str = match token_subject {
        serde_json::Value::String(s) => s,
        _ => return Err(AppError::TokenPayloadError),
    };

    req.extensions_mut().insert(token_subject_str.to_owned());

    Ok(next.run(req).await)
}
