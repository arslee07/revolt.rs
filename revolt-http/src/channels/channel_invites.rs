use revolt_models::channel::Invite;

use crate::prelude::*;

impl RevoltHttp {
    /// Creates an invite to this channel.
    ///
    /// Channel must be a [Channel::TextChannel].
    pub async fn create_invite(&self, id: impl Into<String>) -> Result<Invite> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/invites", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
