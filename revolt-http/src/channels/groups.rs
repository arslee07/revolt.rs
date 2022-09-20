use crate::prelude::*;
use revolt_models::{channel::Channel, payload::CreateGroupPayload, user::User};

impl RevoltHttp {
    /// Retrieves all users who are part of this group.
    pub async fn fetch_group_members(&self, id: impl Into<String>) -> Result<Vec<User>> {
        Ok(self
            .client
            .get(ep!(self, "/channels/{}/members", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Create a new group channel.
    pub async fn create_group(&self, payload: CreateGroupPayload) -> Result<Channel> {
        Ok(self
            .client
            .post(ep!(self, "/channels/create"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Adds another user to the group.
    pub async fn add_member_to_group(
        &self,
        group_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/recipients/{}",
                group_id.into(),
                member_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }

    /// Removes a user from the group.
    pub async fn remove_member_from_group(
        &self,
        group_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/recipients/{}",
                group_id.into(),
                member_id.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?;
        Ok(())
    }
}
