use crate::prelude::*;
use revolt_models::{
    payload::{ChangeUsernamePayload, EditUserPayload},
    user::{User, UserProfile},
};

impl RevoltHttp {
    /// Retrieve your user information.
    pub async fn fetch_self(&self) -> Result<User> {
        Ok(self
            .client
            .get(ep!(self, "/users/@me"))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Edit currently authenticated user.
    pub async fn edit_user(&self, payload: EditUserPayload) -> Result<User> {
        Ok(self
            .client
            .patch(ep!(self, "/users/@me"))
            .auth(&self.authentication)
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Fetch a user's information.
    pub async fn fetch_user(&self, id: impl Into<String>) -> Result<User> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Change your username.
    pub async fn change_username(&self, payload: ChangeUsernamePayload) -> Result<User> {
        Ok(self
            .client
            .patch(ep!(self, "/users/@me/username"))
            .auth(&self.authentication)
            .json(&payload)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// This returns a default avatar based on the given id.
    pub async fn fetch_default_avatar(&self, id: impl Into<String>) -> Result<Vec<u8>> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}/default_avatar", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .bytes()
            .await?
            .into())
    }

    /// Retrieve a user's profile data.
    ///
    ///Will fail if you do not have permission to access the other user's profile.
    pub async fn fetch_user_profile(&self, id: impl Into<String>) -> Result<UserProfile> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}/profile", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
