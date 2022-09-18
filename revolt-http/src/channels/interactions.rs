use revolt_models::payload::RemoveReactionToMessagePayload;

use crate::prelude::*;

impl RevoltHttp {
    /// React to a given message.
    pub async fn add_reaction_to_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
        emoji: impl Into<String>,
    ) -> Result<()> {
        self.client
            .put(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.into(),
                message_id.into(),
                emoji.into()
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Remove your own, someone else's or all of a given reaction.
    ///
    /// Requires [Permission::ManageMessages] if changing others' reactions.
    pub async fn remove_reaction_to_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
        emoji: impl Into<String>,
        payload: RemoveReactionToMessagePayload,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions/{}",
                channel_id.into(),
                message_id.into(),
                emoji.into()
            ))
            .query(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Remove your own, someone else's or all of a given reaction.
    ///
    /// Requires [Permission::ManageMessages].
    pub async fn remove_all_reactions_from_message(
        &self,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Result<()> {
        self.client
            .delete(ep!(
                self,
                "/channels/{}/messages/{}/reactions",
                channel_id.into(),
                message_id.into(),
            ))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }
}
