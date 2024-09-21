use crate::{
    constants::{
        ACCESS_TOKEN_COOKIE_NAME, ACCESS_TOKEN_MAX_AGE, REFRESH_TOKEN_COOKIE_NAME,
        REFRESH_TOKEN_MAX_AGE,
    },
    errors::AppError,
    graphql::auth::TokenVerifier,
    services::auth::AuthService,
    types::{AuthResponse, LoginInput},
    Provider,
};
use actix_web::cookie::{Cookie, SameSite};
use async_graphql::{Context, ErrorExtensions, Object, Result};
use time::Duration;

#[derive(Default)]
pub(super) struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthResponse> {
        let username = ascii85::decode(&input.username).map_err(|e| {
            tracing::error!("Error while decoding username: {:?}", e);
            AppError::BadInput("Invalid username".to_string()).extend()
        })?;
        let username = String::from_utf8(username).map_err(|e| {
            tracing::error!("Error while converting username to string: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let password = ascii85::decode(&input.password).map_err(|e| {
            tracing::error!("Error while decoding password: {:?}", e);
            AppError::BadInput("Invalid password".to_string()).extend()
        })?;
        let password = String::from_utf8(password).map_err(|e| {
            tracing::error!("Error while converting password to string: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let service = provider.provide::<AuthService>();
        let user_creds = service
            .login(&username, &password)
            .await
            .map_err(|e| e.extend())?;

        let access_token_cookie = Cookie::build(
            ACCESS_TOKEN_COOKIE_NAME,
            &user_creds.credential.access_token,
        )
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(ACCESS_TOKEN_MAX_AGE)
        .finish();

        let refresh_token_cookie = Cookie::build(
            REFRESH_TOKEN_COOKIE_NAME,
            &user_creds.credential.refresh_token,
        )
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(REFRESH_TOKEN_MAX_AGE)
        .finish();

        ctx.append_http_header("Set-Cookie", access_token_cookie.to_string());
        ctx.append_http_header("Set-Cookie", refresh_token_cookie.to_string());

        Ok(user_creds)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let access_token_cookie = Cookie::build(ACCESS_TOKEN_COOKIE_NAME, "")
            .path("/")
            .secure(false)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::ZERO)
            .finish();

        let refresh_token_cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, "")
            .path("/")
            .secure(false)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::ZERO)
            .finish();

        ctx.append_http_header("Set-Cookie", access_token_cookie.to_string());
        ctx.append_http_header("Set-Cookie", refresh_token_cookie.to_string());

        Ok(true)
    }

    async fn refresh_token(&self, ctx: &Context<'_>) -> Result<AuthResponse> {
        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let token_verifier = ctx.data::<TokenVerifier>().map_err(|e| {
            tracing::error!("Failed to get token verifier: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let refresh_token = token_verifier.get_verified_refresh_token(ctx).await?;
        let auth_service = provider.provide::<AuthService>();
        let user_creds = auth_service
            .refresh_token(&refresh_token)
            .await
            .map_err(|e| match e {
                AppError::ResourceNotFound(_) => AppError::Unauthenticated.extend(),
                e => e.extend(),
            })?;

        let access_token_cookie = Cookie::build(
            ACCESS_TOKEN_COOKIE_NAME,
            &user_creds.credential.access_token,
        )
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(ACCESS_TOKEN_MAX_AGE)
        .finish();

        let refresh_token_cookie = Cookie::build(
            REFRESH_TOKEN_COOKIE_NAME,
            &user_creds.credential.refresh_token,
        )
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(REFRESH_TOKEN_MAX_AGE)
        .finish();

        ctx.append_http_header("Set-Cookie", access_token_cookie.to_string());
        ctx.append_http_header("Set-Cookie", refresh_token_cookie.to_string());

        Ok(user_creds)
    }
}
