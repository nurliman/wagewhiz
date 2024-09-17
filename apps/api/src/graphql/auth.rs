use crate::{
    constants::{ACCESS_TOKEN_COOKIE_NAME, REFRESH_TOKEN_COOKIE_NAME},
    entities::user_accounts::Model as UserAccount,
    errors::AppError,
    services::{token::TokenService, users::UsersService},
    Provider,
};
use actix_web::{http::header::AUTHORIZATION, HttpRequest};
use async_graphql::{Context, ErrorExtensions, Result};
use uuid::Uuid;

pub struct TokenVerifier {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl TokenVerifier {
    // construct from actix http request
    pub fn from_http_request(request: &HttpRequest) -> Self {
        // Get the access token from the cookie
        let access_token = request
            .cookie(ACCESS_TOKEN_COOKIE_NAME)
            .map(|c| c.value().to_string());

        // Get the access token from the authorization header
        let access_token = access_token.or_else(|| {
            request
                .headers()
                .get(AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_string())
                    } else {
                        Some(auth_value.to_string())
                    }
                })
        });

        // Get the refresh token from the cookie
        let refresh_token = request
            .cookie(REFRESH_TOKEN_COOKIE_NAME)
            .map(|c| c.value().to_string());

        Self {
            access_token,
            refresh_token,
        }
    }

    pub async fn verify_access_token(&self, ctx: &Context<'_>) -> Result<Uuid> {
        let token = self
            .access_token
            .clone()
            .ok_or(AppError::Unauthenticated.extend())?;

        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;

        provider
            .provide::<TokenService>()
            .verify_access_token(&token)
            .await
            .map_err(|e| e.extend())
    }

    pub async fn get_verified_refresh_token(&self, ctx: &Context<'_>) -> Result<String> {
        let refresh_token = self
            .refresh_token
            .clone()
            .ok_or(AppError::Unauthenticated.extend())?;

        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;

        provider
            .provide::<TokenService>()
            .verify_refresh_token(&refresh_token)
            .await
            .map_err(|e| e.extend())?;

        Ok(refresh_token)
    }

    pub async fn get_verified_user(&self, ctx: &Context<'_>) -> Result<UserAccount> {
        let user_id = self.verify_access_token(ctx).await?;

        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;

        provider
            .provide::<UsersService>()
            .find_by_id(user_id)
            .await
            .map_err(|_| AppError::Forbidden.extend())
    }
}
