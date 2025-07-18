use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct ProductListResponse {
    pub id: i32,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_category: Option<String>,
    pub assets_path: Option<String>
}