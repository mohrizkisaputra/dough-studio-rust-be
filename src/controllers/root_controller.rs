use axum::Json;
use crate::models::root_models::RootControllerResponse;

pub async fn hello() -> Json<RootControllerResponse> {
    Json(RootControllerResponse::new(
        true,
        String::from("Hello From Root!"),
    ))
}