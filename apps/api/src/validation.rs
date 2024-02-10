use crate::errors::AppError;
use axum::{
    async_trait,
    extract::{
        rejection::{FormRejection, JsonRejection},
        Form, FromRequest, Request,
    },
    http::header::CONTENT_TYPE,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct JsonOrForm<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(value) = Json::<T>::from_request(req, state).await?;
                value.validate()?;
                return Ok(JsonOrForm(value));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(value) = Form::<T>::from_request(req, state).await?;
                value.validate()?;
                return Ok(JsonOrForm(value));
            }
        }

        Err(AppError::UnsupportedMediaType)
    }
}
