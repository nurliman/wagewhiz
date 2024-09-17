pub mod auth;
// pub mod extension;
pub mod mutation;
pub mod query;
pub mod schema;

use actix_web::{get, route, web, HttpRequest, Responder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use auth::TokenVerifier;
use schema::AppSchema;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql_playground);
    cfg.service(graphql_handler);
}

#[route("/graphql", method = "POST", method = "OPTIONS")]
async fn graphql_handler(
    schema: web::Data<AppSchema>,
    request: HttpRequest,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut graphql_request = graphql_request.into_inner();
    graphql_request = graphql_request.data(TokenVerifier::from_http_request(&request));
    schema.execute(graphql_request).await.into()
}

// GraphiQL playground UI
#[get("/graphql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}
