use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub enum ErrorId {
    LabelMe,
    InternalError,
    InvalidSenssion,
    OnboardingNotFinished,
    AlreadyAuthenticated,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Error {
        error: ErrorId,
    },
    Authenticated,
    Pong {
        data: u32,
    },
    Message {
        #[serde(flatten)]
        message: Message,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ClientToServerEvent {
    Authenticate { token: String },
    Ping { data: i32 },
    BeginTyping { channel: String },
    EndTyping { channel: String },
}
