use revolt_models::payload::SendMessagePayload;

#[derive(Clone, Debug)]
pub struct SendMessagePayloadBuilder {
    content: String,
}

impl SendMessagePayloadBuilder {
    pub fn new() -> Self {
        SendMessagePayloadBuilder {
            content: String::default(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn build(self) -> SendMessagePayload {
        SendMessagePayload {
            content: self.content,
        }
    }
}
