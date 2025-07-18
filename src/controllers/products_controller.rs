use std::path;

use axum::{ Router, routing::get, Extension};
use crate::helper::graphql_helper::{graphql_playground};

pub struct ProductsController {
    app: Router,
}

impl ProductsController {
    pub fn default() -> ProductsController {
        ProductsController {
            app: Router::new()
                .route("/product_list_all", get(graphql_playground()).get(product_list_all)).layer(Extension(create_schema())),
        }
    }

    pub fn app(self) -> Router {
        self.app
    }
}