use async_graphql::{Schema, EmptySubscription};
use crate::graphql::{query::QueryRoot, mutation::MutationRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription).finish()
}
