use async_graphql::SimpleObject;
use crate::models::products_models::{ProductListResponse};

#[derive(SimpleObject)]
pub struct ProductListGQL {
    pub id: i32,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_category: Option<String>,
    pub assets_path: Option<String>
}

impl From<ProductListResponse> for ProductListGQL {
    fn from(p: ProductListResponse) -> Self {
        Self {
            id: p.id,
            product_name: p.product_name,
            product_description: p.product_description,
            product_category: p.product_category,
            assets_path: p.assets_path,
        }
    }
}
