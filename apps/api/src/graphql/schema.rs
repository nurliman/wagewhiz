use super::{mutation::Mutation, query::Query};
use crate::Provider;
use async_graphql::{extensions::Logger, EmptySubscription, Schema};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(provider: Provider) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(Logger)
        .data(provider)
        .finish()
}
