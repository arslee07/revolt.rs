mod channel_information;
mod core;
mod server_information;
mod user_information;

use reqwest::{Client, StatusCode};
use revolt_models::{
    authentication::Authentication, message::Message, payload::SendMessagePayload, ApiError,
};
use std::result::Result as StdResult;

type Result<T> = StdResult<T, RevoltHttpError>;

pub(crate) mod prelude {
    pub(crate) use crate::{ep, RequestBuilderExt, ResponseExt, Result, RevoltHttp};
}

#[derive(Debug, thiserror::Error)]
pub enum RevoltHttpError {
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Error while processing an HTTP request: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Error returned from API")]
    Api(ApiError),
}

#[macro_export]
macro_rules! ep {
    ($self:ident, $ep:literal, $($args:tt)*) => {
        format!(concat!("{}", $ep), $self.base_url, $($args)*)
    };

    ($self:ident, $ep:literal) => {
        format!(concat!("{}", $ep), $self.base_url)
    };

    (api_root = $api_root:expr, $ep:literal $($args:tt)*) => {
        format!(concat!("{}", $ep), $api_root, $($args)*)
    };
}

trait RequestBuilderExt {
    fn auth(self, authentication: &Authentication) -> Self;
}

impl RequestBuilderExt for reqwest::RequestBuilder {
    fn auth(self, authentication: &Authentication) -> Self {
        self.header(authentication.header_key(), authentication.value())
    }
}

#[async_trait::async_trait]
trait ResponseExt {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl ResponseExt for reqwest::Response {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized,
    {
        let status = self.status();

        if status.is_success() {
            Ok(self)
        } else {
            // NOTE: it's a workaround thing but there are no alternative methods
            // because API returns HTML instead of parseable JSON
            // uhhhm also sorry for my broken english
            if status == StatusCode::UNAUTHORIZED {
                return Err(RevoltHttpError::Api(ApiError::Unauthenticated));
            }

            Err(RevoltHttpError::Api(ApiError::from(self.json().await?)))
        }
    }
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
    pub async fn send_message(
        &self,
        channel_id: String,
        payload: SendMessagePayload,
    ) -> Result<Message> {
        Ok(self
            .client
            .post(ep!(self, "/channels/{}/messages", channel_id))
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
