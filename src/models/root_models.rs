use serde::Serialize;

#[derive(Serialize)]
pub struct RootControllerResponse {
    is_successful: bool,
    root_message: String,
}

impl RootControllerResponse {
    pub fn new(_is_successful: bool, _root_message: String) -> Self {
        Self {
            is_successful: _is_successful,
            root_message: _root_message,
        }
    }
}
