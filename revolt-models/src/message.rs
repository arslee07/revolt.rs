use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub system: Option<SystemMessage>,
    pub channel: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessage {
    Text { content: String },
    UserAdded { id: String, by: String },
    UserRemove { id: String, by: String },
    UserJoined { id: String },
    UserLeft { id: String },
    UserKicked { id: String },
    UserBanned { id: String },
    ChannelRenamed { name: String, by: String },
    ChannelDescriptionChanged { by: String },
    ChannelIconChanged { by: String },
}
