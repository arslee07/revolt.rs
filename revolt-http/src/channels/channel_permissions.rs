use crate::prelude::*;
use revolt_models::{
    channel::Channel,
    payload::{SetDefaultPermissionPayload, SetRolePermissionPayload},
};

impl RevoltHttp {
    /// Sets permissions for the specified role in this channel.
    ///
    /// Channel must be a [Channel::TextChannel] or [Channel::VoiceChannel].
    pub async fn set_role_permissions(
        &self,
        channel_id: impl Into<String>,
        role_id: impl Into<String>,
        payload: SetRolePermissionPayload,
    ) -> Result<Channel> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/channels/{}/permissions/{}",
                channel_id.into(),
                role_id.into()
            ))
            .auth(&self.authentication)
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Sets permissions for the specified role in this channel.
    ///
    /// Channel must be a [Channel::Group], [Channel::TextChannel] or [Channel::VoiceChannel].
    pub async fn set_default_permissions(
        &self,
        channel_id: impl Into<String>,
        payload: SetDefaultPermissionPayload,
    ) -> Result<Channel> {
        Ok(self
            .client
            .put(ep!(
                self,
                "/channels/{}/permissions/default",
                channel_id.into(),
            ))
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
