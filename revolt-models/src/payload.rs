use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct SendMessagePayload {
    pub content: String,
}
