use crate::{
    entities::{people, user_accounts},
    errors::AppError,
    services,
};
use axum::{Extension, Json};

pub async fn get_me(
    Extension(user_id): Extension<String>,
) -> Result<Json<user_accounts::Model>, AppError> {
    let user = services::users::get_user_by_id(&user_id).await?;
    Ok(Json(user))
}

pub async fn get_my_personal_info(
    Extension(user_id): Extension<String>,
) -> Result<Json<people::Model>, AppError> {
    let user = services::users::get_user_by_id(&user_id).await?;

    match user.person_id {
        Some(person_id) => {
            let person = services::people::get_person_by_id(&person_id.to_string()).await?;
            Ok(Json(person))
        }
        None => Err(AppError::AccountNotLinkedToPerson),
    }
}
