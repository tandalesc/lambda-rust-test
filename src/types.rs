use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestEvent {
    #[serde(rename = "firstName")]
    pub first_name: String
}

#[derive(Serialize)]
pub struct HandlerResponse {
    pub message: String
}
