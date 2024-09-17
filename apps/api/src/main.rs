mod config;
mod constants;
mod database;
mod entities;
mod errors;
mod graphql;
mod repositories;
mod services;
mod types;
mod utils;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use config::ConfigProvider;
use database::DatabaseProvider;
use listenfd::ListenFd;
use nject::provider;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[provider]
struct Provider {
    #[provide]
    config_provider: ConfigProvider,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the configuration
    let config_provider = ConfigProvider {};
    let config = config_provider.get_config();

    // Set up logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,actix_server=warn,actix_web=warn,async-graphql=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let provider = Provider { config_provider };
    let db: DatabaseProvider = provider.provide();
    let db = db.get_db_conn().await;

    // Migrate the database if configured to do so
    if config.db_migration {
        database::migrate(db).await.unwrap();
    }

    let schema = graphql::schema::build_schema(provider).await;

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        http::header::ACCEPT,
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .configure(graphql::route)
    });

    let server_url = format!("{}:{}", config.host, config.port);

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await
}
