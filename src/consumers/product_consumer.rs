use async_graphql::{Context, Object};
use sqlx::PgPool;
use crate::types::product::ProductListGQL;
use crate::repository::products_repository::get_all_products;

#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn products(&self, ctx: &Context<'_>) -> Vec<ProductListGQL> {
        let pool = ctx.data_unchecked::<PgPool>();
        let products = get_all_products(pool).await.unwrap_or_default();
        products.into_iter().map(ProductListGQL::from).collect()
    }
}
