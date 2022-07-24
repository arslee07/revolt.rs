use reqwest::{Client, Method, StatusCode};
use revolt_models::{
    authentication::Authentication, message::Message, payload::SendMessagePayload, ApiError,
};
use serde::Serialize;

type Result<T> = std::result::Result<T, RevoltHttpError>;

#[derive(Debug, thiserror::Error)]
pub enum RevoltHttpError {
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Error while processing an HTTP request: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Error returned from API")]
    Api(ApiError),
}

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

        let resp = r.send().await?;
        let status = resp.status();
        let body = resp.text().await?;

        if status.is_success() {
            let json = serde_json::from_str::<R>(&body)?;
            Ok(json)
        } else {
            // NOTE: it's a workaround thing but there are no alternative methods
            // because API returns HTML instead of parseable JSON
            // uhhhm also sorry for my broken english
            if status == StatusCode::UNAUTHORIZED {
                return Err(RevoltHttpError::Api(ApiError::Unauthenticated));
            }

            let json = serde_json::from_str::<ApiError>(&body)?;
            Err(RevoltHttpError::Api(ApiError::from(json)))
        }
    }

    pub async fn send_message(
        &self,
        channel_id: String,
        payload: SendMessagePayload,
    ) -> Result<()> {
        self.req(
            "POST",
            format!("/channels/{}/messages", channel_id).as_str(),
            Some(payload),
        )
        .await
    }
}
