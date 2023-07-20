mod db;
mod env;
mod errors;
mod handlers;
mod models;
mod schema;
mod services;
mod validation;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let env_var = env::get_env_var().unwrap().clone();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::get_pool().await.unwrap().clone();

    let app = Router::new()
        .route("/v0/users", get(handlers::auth::list_users))
        .route("/v0/users", post(create_user))
        .route("/v0/auth/sign-in", post(handlers::auth::sign_in))
        .with_state(pool);

    let addr = format!("{}:{}", env_var.host, env_var.port)
        .parse::<SocketAddr>()
        .map_err(|e| {
            tracing::error!("Error parsing address: {:?}", e);
            errors::AppError::AddrError(e)
        })
        .unwrap();

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(
    State(pool): State<db::Pool>,
    Json(new_user): Json<models::NewUser>,
) -> Result<Json<models::User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = diesel::insert_into(schema::users::table)
        .values(new_user)
        .returning(models::User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    db::Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = db::Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

// async fn list_users(
//     DatabaseConnection(mut conn): DatabaseConnection,
// ) -> Result<Json<Vec<models::User>>, (StatusCode, String)> {
//     let res = schema::users::table
//         .select(models::User::as_select())
//         .load(&mut conn)
//         .await
//         .map_err(internal_error)?;
//     Ok(Json(res))
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
