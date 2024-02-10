use crate::{
    constants::ACCESS_TOKEN_COOKIE_NAME, errors::AppError, services::auth::verify_access_token,
};
use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn auth(
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        Some(auth_value.to_owned())
                    }
                })
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

    // TODO: create a struct for app state
    req.extensions_mut().insert(token_subject_str.to_owned());

    Ok(next.run(req).await)
}
