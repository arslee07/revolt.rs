use crate::prelude::*;
use revolt_models::user::User;

impl RevoltHttp {
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
}
