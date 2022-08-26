use crate::prelude::*;
use revolt_models::channel::Channel;

impl RevoltHttp {
    pub async fn fetch_channel(&self, id: impl Into<String>) -> Result<Channel> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
