use async_graphql::MergedObject;

use crate::consumer::product_consumer::ProductQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProductQuery);