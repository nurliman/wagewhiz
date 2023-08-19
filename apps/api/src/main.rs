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
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
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
            "/v0/me",
            get(handlers::me::get_me).route_layer(middleware::from_fn(middlewares::auth)),
        )
        .route(
            "/v0/me/personal-info",
            get(handlers::me::get_my_personal_info)
                .route_layer(middleware::from_fn(middlewares::auth)),
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
        .layer(
            CorsLayer::new()
                // TODO: change this to the frontend url dynamically using env var
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
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
