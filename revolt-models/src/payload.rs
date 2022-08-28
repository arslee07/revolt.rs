use serde::Serialize;

use crate::{
    channel::FieldsChannel,
    embed::SendableEmbed,
    message::{Interactions, Masquerade, MessageSort, Reply},
    permission::{Override, Permission},
    user::{FieldsUser, PartialUserProfile, UserStatus},
};

#[derive(Serialize, Debug, Clone)]
pub struct SendMessagePayload {
    /// Message content to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Attachments to include in message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    /// Messages to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Reply>>,
    /// Embeds to include in message
    ///
    /// Text embed content contributes to the content length cap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Masquerade to apply to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
    /// Information about how this message should be interacted with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Interactions>,
}

/// User data
#[derive(Serialize, Debug, Clone)]
pub struct EditUserPayload {
    /// New user status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<PartialUserProfile>,
    /// Attachment ID for avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Fields to remove from user object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsUser>>,
}

/// Change username data
#[derive(Serialize, Debug, Clone)]
pub struct ChangeUsernamePayload {
    /// New username
    pub username: String,
    /// Current username password
    pub password: String,
}

/// Send friend request data
#[derive(Serialize, Debug, Clone)]
pub struct SendFriendRequestPayload {
    /// Friend's usernane
    pub username: String,
}

/// Edit channel data
#[derive(Serialize, Debug, Clone)]
pub struct EditChannelPayload {
    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Channel description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Group owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Icon attachment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Whether this channel is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Fields to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsChannel>>,
}

/// Set role permission payload data
#[derive(Serialize, Debug, Clone)]
pub struct SetRolePermissionPayload {
    /// Representation of a single permission override
    pub permissions: Override,
}

/// Set role permission payload data
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SetDefaultPermissionPayload {
    Value {
        /// Permission values to set for members in a [Channel::Group]
        permissions: Permission,
    },
    Field {
        /// Allow / deny values to set for members in this [Channels::TextChannel] or [Channels::VoiceChannel]
        permissions: Override,
    },
}

/// Query parameters
#[derive(Serialize, Debug, Clone)]
pub struct FetchMessagesPayload {
    /// Maximum number of messages to fetch
    ///
    /// For fetching nearby messages, this is `(limit + 1)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Message sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,
    /// Message id to search around
    ///
    /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
    /// It will also take half of limit rounded as the limits to each side.
    /// It also fetches the message ID specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearby: Option<String>,
    /// Whether to include user (and member, if server channel) objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

/// Search Parameters
#[derive(Serialize, Debug, Clone)]
pub struct SearchForMessagesPayload {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    pub query: String,

    /// Maximum number of messages to fetch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by relevance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,
    /// Whether to include user (and member, if server channel) objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

/// Message details
#[derive(Serialize, Debug, Clone)]
pub struct EditMessagePayload {
    /// New message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Embeds to include in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
}

/// Search parameters
#[derive(Serialize, Debug, Clone)]
pub struct BulkDeleteMessagesPayload {
    /// Message IDs
    pub ids: Vec<String>,
}
