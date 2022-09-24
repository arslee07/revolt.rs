use serde::{Deserialize, Serialize};

use crate::{attachment::Attachment, user::User};

#[allow(dead_code)]
fn if_false(t: &bool) -> bool {
    !t
}

bitflags::bitflags! {
    /// User badge bitfield
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct BotFlags: u64 {
        const Verified = 1;
        const Official = 2;
    }
}

/// Public bot
#[derive(Deserialize, Debug, Clone)]
pub struct PublicBot {
    /// Bot Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Bot Username
    pub username: String,
    /// Profile Avatar
    pub avatar: Option<Attachment>,
    /// Profile Description
    pub description: Option<String>,
}

/// Representation of a bot on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Bot {
    /// Bot Id
    ///
    /// This equals the associated bot user's id.
    #[serde(rename = "_id")]
    pub id: String,
    /// User Id of the bot owner
    pub owner: String,
    /// Token used to authenticate requests for this bot
    pub token: String,
    /// Whether the bot is public
    /// (may be invited by anyone)
    pub public: bool,

    /// Whether to enable analytics
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    /// Whether this bot should be publicly discoverable
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
    /// Reserved; URL for handling interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<String>,
    /// URL for terms of service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    /// URL for privacy policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// Enum of bot flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<BotFlags>,
}

/// Partial representation of a bot on Revolt
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PartialBot {
    /// Bot Id
    ///
    /// This equals the associated bot user's id.
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// User Id of the bot owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Token used to authenticate requests for this bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Whether the bot is public
    /// (may be invited by anyone)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,

    /// Whether to enable analytics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Whether this bot should be publicly discoverable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
    /// Reserved; URL for handling interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<String>,
    /// URL for terms of service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    /// URL for privacy policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// Enum of bot flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<BotFlags>,
}

/// Owned bot.
///
/// Contains bot and user information.
#[derive(Deserialize, Debug, Clone)]
pub struct OwnedBot {
    /// Bot object
    pub bot: Bot,
    /// User object
    pub user: User,
}

/// Optional fields on bot object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsBot {
    Token,
    InteractionsURL,
}

/// Owned bots.
///
/// Both lists are sorted by their IDs.
#[derive(Deserialize, Debug, Clone)]
pub struct OwnedBots {
    /// Bot objects
    pub bots: Vec<Bot>,
    /// User objects
    pub users: Vec<User>,
}
