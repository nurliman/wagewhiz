mod auth_mutation;

use auth_mutation::AuthMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AuthMutation);
