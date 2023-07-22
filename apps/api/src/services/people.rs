use crate::{db, errors::AppError, models::Person, schema::people};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get_people() -> Result<Vec<Person>, AppError> {
    let mut conn = db::get_connection().await?;
    let people = people::table
        .select(Person::as_select())
        .limit(5)
        .load(&mut *conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting people: {:?}", e);
            AppError::InternalError
        })?;

    Ok(people)
}
