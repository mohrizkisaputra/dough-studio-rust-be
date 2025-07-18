use axum::{Router, routing::get};
mod controllers;
mod models;
mod helper;
mod consumers;
mod query;
mod repository;
mod types;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .nest(
            "/product",
            controllers::products_controller::ProductsController::default().app(),
        );
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}