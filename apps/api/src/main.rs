mod constants;
mod db;
mod entities;
mod env;
mod errors;
mod handlers;
mod middlewares;
mod services;
mod types;
mod utils;
mod validation;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderName, HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, propagate_header::PropagateHeaderLayer, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let env_var = env::get_env_var().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact(),
        )
        .init();

    let app = Router::new()
        .route("/v0/auth/login", post(handlers::auth::login))
        .route("/v0/auth/logout", post(handlers::auth::logout))
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
        // High level logging of requests and responses
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new())
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        // Add CORS headers to responses
        .layer(
            CorsLayer::new()
                // TODO: change this to the frontend url dynamically using env var
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:8080".parse::<HeaderValue>().unwrap(),
                ])
                .allow_credentials(true)
                .allow_headers([
                    ACCEPT,
                    AUTHORIZATION,
                    CONTENT_TYPE,
                    HeaderName::from_static("x-request-id"),
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ]),
        );

    let app = app.fallback(handlers::mod_404::not_found);

    let addr = format!("{}:{}", env_var.host, env_var.port)
        .parse::<SocketAddr>()
        .map_err(|e| {
            tracing::error!("Error parsing address: {:?}", e);
            errors::AppError::AddrError(e)
        })
        .unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
