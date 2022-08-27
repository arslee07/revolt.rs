use crate::prelude::*;
use revolt_models::{message::Message, payload::SendMessagePayload};

impl RevoltHttp {
    pub async fn send_message(
        &self,
        channel_id: String,
        payload: SendMessagePayload,
    ) -> Result<Message> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/messages", channel_id))
            .auth(&self.authentication)
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
