use axum::Json;
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};
use async_graphql::{Schema, Object, EmptySubscription, Context};

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn r#first(&self, _ctx: &Context<'_>) -> &str {
        "Hai. this is first try implement GraphQl"
    }

    async fn r#try(&self, _ctx: &Context<'_>) -> &str {
        "Coba coba cob"
    }
}

#[Object]
impl MutationRoot {
    async fn echo(&self, _ctx: &Context<'_>, message: String) -> String {
        format!("Echo: {}", message)
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

pub fn graphql_playground() -> axum::response::Html<String> {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

