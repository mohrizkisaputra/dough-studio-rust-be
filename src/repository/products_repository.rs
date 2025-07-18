use sqlx::PgPool;
use crate::models::product_models::{ProductListResponse};

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<ProductListResponse>, sqlx::Error> {
    sqlx::query_as!(
        ProductListResponse,
        "SELECT id, product_name, product_description, product_category, assets_path FROM products"
    )
    .fetch_all(pool)
    .await
}

