mod me_query;
mod people_query;
mod users_query;

use me_query::MeQuery;
use people_query::PeopleQuery;
use users_query::UsersQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(MeQuery, PeopleQuery, UsersQuery);
