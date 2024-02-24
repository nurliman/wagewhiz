use crate::{
    constants::{
        ACCESS_TOKEN_COOKIE_NAME, ACCESS_TOKEN_MAX_AGE, REFRESH_TOKEN_COOKIE_NAME,
        REFRESH_TOKEN_MAX_AGE,
    },
    errors::AppError,
    models::{RefreshToken, SignIn},
    services,
    validation::JsonOrForm,
};
use axum::{
    http::{header, Response},
    response::IntoResponse,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use serde_json::json;

pub async fn sign_in(JsonOrForm(body): JsonOrForm<SignIn>) -> Result<impl IntoResponse, AppError> {
    let username = ascii85::decode(&body.username).map_err(|e| {
        tracing::error!("Error while decoding username: {:?}", e);
        AppError::InvalidBase85Encoding
    })?;
    let username = String::from_utf8(username).map_err(|e| {
        tracing::error!("Error while converting username to string: {:?}", e);
        AppError::InternalError
    })?;

    let password = ascii85::decode(&body.password).map_err(|e| {
        tracing::error!("Error while decoding password: {:?}", e);
        AppError::InvalidBase85Encoding
    })?;
    let password = String::from_utf8(password).map_err(|e| {
        tracing::error!("Error while converting password to string: {:?}", e);
        AppError::InternalError
    })?;

    let user = services::auth::sign_in(&username, &password).await?;

    let access_token_cookie = create_access_token_cookie(&user.credential.access_token);
    let refresh_token_cookie = create_refresh_token_cookie(&user.credential.refresh_token);

    let response = Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::SET_COOKIE, access_token_cookie.to_string())
        .header(header::SET_COOKIE, refresh_token_cookie.to_string())
        .body(json!(user).to_string())
        .map_err(|error| {
            tracing::error!("Error while building response: {:?}", error);
            AppError::InternalError
        })?;

    Ok(response)
}

pub async fn sign_out() -> Result<impl IntoResponse, AppError> {
    let null_access_token_cookie = create_token_cookie(
        ACCESS_TOKEN_COOKIE_NAME,
        "",
        Some(time::Duration::hours(-1)),
    );
    let null_refresh_token_cookie = create_token_cookie(
        REFRESH_TOKEN_COOKIE_NAME,
        "",
        Some(time::Duration::hours(-1)),
    );

    let response = Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::SET_COOKIE, null_access_token_cookie.to_string())
        .header(header::SET_COOKIE, null_refresh_token_cookie.to_string())
        .body(json!({"status": "success"}).to_string())
        .map_err(|error| {
            tracing::error!("Error while building response: {:?}", error);
            AppError::InternalError
        })?;

    Ok(response)
}

pub async fn refresh_token(
    cookie_jar: CookieJar,
    JsonOrForm(body): JsonOrForm<RefreshToken>,
) -> Result<impl IntoResponse, AppError> {
    // get refresh token from cookie
    let refresh_token = cookie_jar
        .get(REFRESH_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    // get refresh token from body
    let refresh_token = refresh_token.or_else(|| body.refresh_token.clone());

    // throw error if refresh token is not found
    let refresh_token = refresh_token.ok_or(AppError::RefreshTokenNotFound)?;

    // regenerate access token and refresh token
    let user = services::auth::refresh_token(&refresh_token).await?;

    let new_access_token_cookie = create_access_token_cookie(&user.credential.access_token);
    let new_refresh_token_cookie = create_refresh_token_cookie(&user.credential.refresh_token);

    let response = Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::SET_COOKIE, new_access_token_cookie.to_string())
        .header(header::SET_COOKIE, new_refresh_token_cookie.to_string())
        .body(json!(user).to_string())
        .map_err(|error| {
            tracing::error!("Error while building response: {:?}", error);
            AppError::InternalError
        })?;

    Ok(response)
}

// TODO: Set cookie domain and path based on environment
// TODO: Set cookie secure based on environment
fn create_access_token_cookie(access_token: &str) -> Cookie<'_> {
    create_token_cookie(
        ACCESS_TOKEN_COOKIE_NAME,
        access_token,
        Some(ACCESS_TOKEN_MAX_AGE),
    )
}

// TODO: Set cookie domain and path based on environment
// TODO: Set cookie secure based on environment
fn create_refresh_token_cookie(refresh_token: &str) -> Cookie<'_> {
    create_token_cookie(
        REFRESH_TOKEN_COOKIE_NAME,
        refresh_token,
        Some(REFRESH_TOKEN_MAX_AGE),
    )
}

fn create_token_cookie<'a>(
    key: &'a str,
    token: &'a str,
    max_age: Option<time::Duration>,
) -> Cookie<'a> {
    let cookie = Cookie::build((key, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax);

    let cookie = if let Some(max_age) = max_age {
        cookie.max_age(max_age)
    } else {
        cookie
    };

    cookie.build()
}
