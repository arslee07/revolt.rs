use crate::prelude::*;
use revolt_models::{
    payload::SendFriendRequestPayload,
    user::{Mutuals, User},
};

impl RevoltHttp {
    /// This fetches your direct messages, including any DM and group DM conversations.
    pub async fn fetch_mutual_friends_and_servers(&self, id: impl Into<String>) -> Result<Mutuals> {
        Ok(self
            .client
            .get(ep!(self, "/users/{}/mutual", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Accept another user's friend request
    pub async fn accept_friend_request(&self, id: impl Into<String>) -> Result<User> {
        Ok(self
            .client
            .put(ep!(self, "/users/{}/friend", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Denies another user's friend request or removes an existing friend.
    pub async fn remove_or_deny_friend(&self, id: impl Into<String>) -> Result<User> {
        Ok(self
            .client
            .delete(ep!(self, "/users/{}/friend", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Block another user by their id.
    pub async fn block_user(&self, id: impl Into<String>) -> Result<User> {
        Ok(self
            .client
            .put(ep!(self, "/users/{}/block", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Unblock another user by their id.
    pub async fn unblock_user(&self, id: impl Into<String>) -> Result<User> {
        Ok(self
            .client
            .delete(ep!(self, "/users/{}/block", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }

    /// Send a friend request to another user.
    pub async fn send_friend_request(&self, payload: SendFriendRequestPayload) -> Result<User> {
        Ok(self
            .client
            .post(ep!(self, "/users/friend"))
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
