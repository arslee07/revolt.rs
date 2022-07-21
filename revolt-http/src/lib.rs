use reqwest::{Client, Method};
use revolt_models::{
    authentication::Authentication, message::Message, payload::SendMessagePayload,
};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct RevoltHttp {
    base_url: String,
    client: Client,
    authentication: Authentication,
}

impl RevoltHttp {
    pub fn new(authentication: Authentication) -> Self {
        RevoltHttp {
            base_url: "https://api.revolt.chat".to_string(),
            client: Client::new(),
            authentication,
        }
    }

    pub fn new_base_url(authentication: Authentication, base_url: String) -> Self {
        RevoltHttp {
            base_url,
            client: Client::new(),
            authentication,
        }
    }
}

impl RevoltHttp {
    async fn req<B: Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        method: &str,
        path: &str,
        body: Option<B>,
    ) -> Result<R> {
        let mut r = self
            .client
            .request(
                Method::from_bytes(method.as_bytes()).unwrap(),
                format!("{}{}", self.base_url, path),
            )
            .header::<String, String>(
                self.authentication.header_key(),
                self.authentication.value(),
            );

        if let Some(b) = body {
            r = r.json(&b);
        }

        Ok(r.send().await?.json().await?)
    }

    pub async fn send_message(
        &self,
        channel_id: String,
        payload: SendMessagePayload,
    ) -> Result<Message> {
        self.req(
            "POST",
            format!("/channels/{}/messages", channel_id).as_str(),
            Some(payload),
        )
        .await
    }
}
