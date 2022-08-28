use revolt_models::payload::SendMessagePayload;

#[derive(Clone, Debug)]
pub struct SendMessagePayloadBuilder {
    content: Option<String>,
}

impl SendMessagePayloadBuilder {
    pub fn new() -> Self {
        SendMessagePayloadBuilder { content: None }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn build(self) -> SendMessagePayload {
        SendMessagePayload {
            content: self.content,
            // TODO: implement other fields
            attachments: None,
            replies: None,
            embeds: None,
            masquerade: None,
            interactions: None,
        }
    }
}
