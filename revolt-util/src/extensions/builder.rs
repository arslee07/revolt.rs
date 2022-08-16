use revolt_models::payload::SendMessagePayload;

use crate::builders::message::SendMessagePayloadBuilder;

pub trait BuilderExt<T> {
    fn builder() -> T;
}

impl BuilderExt<SendMessagePayloadBuilder> for SendMessagePayload {
    fn builder() -> SendMessagePayloadBuilder {
        SendMessagePayloadBuilder::new()
    }
}
