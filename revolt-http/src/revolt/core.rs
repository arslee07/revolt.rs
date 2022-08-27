use revolt_models::core::InstanceConfiguration;

use crate::prelude::*;

impl RevoltHttp {
    pub async fn query_node(&self) -> Result<InstanceConfiguration> {
        Ok(self
            .client
            .get(ep!(self, "/"))
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
