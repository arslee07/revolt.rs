use crate::prelude::*;
use revolt_models::{
    bot::{Bot, OwnedBot, OwnedBots, PublicBot},
    payload::{CreateBotPayload, EditBotPayload, InviteBotPayload},
};

impl RevoltHttp {
    /// Create a new Revolt bot.
    pub async fn create_bot(&self, payload: CreateBotPayload) -> Result<Bot> {
        Ok(self
            .client
            .post(ep!(self, "/bots/create"))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch details of a public (or owned) bot by its id.
    pub async fn fetch_public_bot(&self, id: impl Into<String>) -> Result<PublicBot> {
        Ok(self
            .client
            .get(ep!(self, "/bots/{}/invite", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Invite a bot to a server or group by its id.
    pub async fn invite_bot(
        &self,
        bot_id: impl Into<String>,
        payload: InviteBotPayload,
    ) -> Result<()> {
        self.client
            .post(ep!(self, "/bots/{}/invite", bot_id.into()))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Fetch details of a bot you own by its id.
    pub async fn fetch_bot(&self, id: impl Into<String>) -> Result<OwnedBot> {
        Ok(self
            .client
            .get(ep!(self, "/bots/{}/invite", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Delete a bot by its id.
    pub async fn delete_bot(&self, id: impl Into<String>) -> Result<()> {
        self.client
            .delete(ep!(self, "/bots/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?;
        Ok(())
    }

    /// Edit bot details by its id.
    pub async fn edit_bot(&self, id: impl Into<String>, payload: EditBotPayload) -> Result<Bot> {
        Ok(self
            .client
            .patch(ep!(self, "/bots/{}", id.into()))
            .json(&payload)
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch all of the bots that you have control over.
    pub async fn fetch_owned_bots(&self) -> Result<OwnedBots> {
        Ok(self
            .client
            .get(ep!(self, "/bots/@me"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
