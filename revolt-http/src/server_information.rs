use crate::prelude::*;
use revolt_models::server::Server;

impl RevoltHttp {
    pub async fn fetch_server(&self, id: impl Into<String>) -> Result<Server> {
        Ok(self
            .client
            .get(ep!(self, "/servers/{}", id.into()))
            .auth(&self.authentication)
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
