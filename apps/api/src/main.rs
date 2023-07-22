mod db;
mod env;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod schema;
mod services;
mod utils;
mod validation;

use axum::{
    middleware,
    routing::{get, post},
    Router,
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
        .route("/v0/auth/sign-in", post(handlers::auth::sign_in))
        .route(
            "/v0/auth/refresh-token",
            post(handlers::auth::refresh_token),
        )
        .route(
            "/v0/people",
            get(handlers::people::get_people).route_layer(middleware::from_fn(middlewares::auth)),
        )
        .route(
            "/v0/people/:person_id",
            get(handlers::people::get_person_by_id)
                .route_layer(middleware::from_fn(middlewares::auth)),
        )
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
