use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{attachment::Attachment, permission::OverrideField};

/// Representation of a channel on Revolt
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// Personal "Saved Notes" channel which allows users to save messages
    SavedMessages {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the user this channel belongs to
        user: String,
    },

    /// Direct message channel between two users
    DirectMessage {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,

        /// Whether this direct message channel is currently open on both sides
        active: bool,
        /// 2-tuple of user ids participating in direct message
        recipients: Vec<String>,
        /// Id of the last message sent in this channel
        last_message_id: Option<String>,
    },

    /// Group channel between 1 or more participants
    Group {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,

        /// Display name of the channel
        name: String,
        /// User id of the owner of the group
        owner: String,
        /// Channel description
        description: Option<String>,
        /// Array of user ids participating in channel
        recipients: Vec<String>,

        /// Custom icon attachment
        icon: Option<Attachment>,
        /// Id of the last message sent in this channel
        last_message_id: Option<String>,

        /// Permissions assigned to members of this group
        /// (does not apply to the owner of the group)
        permissions: Option<i64>,

        /// Whether this group is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },

    /// Text channel belonging to a server
    TextChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the server this channel belongs to
        server: String,

        /// Display name of the channel
        name: String,
        /// Channel description
        description: Option<String>,

        /// Custom icon attachment
        icon: Option<Attachment>,
        /// Id of the last message sent in this channel
        last_message_id: Option<String>,

        /// Default permissions assigned to users in this channel
        default_permissions: Option<OverrideField>,
        /// Permissions assigned based on role to this channel
        #[serde(default = "HashMap::<String, OverrideField>::new")]
        role_permissions: HashMap<String, OverrideField>,

        /// Whether this channel is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },

    /// Voice channel belonging to a server
    VoiceChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the server this channel belongs to
        server: String,

        /// Display name of the channel
        name: String,
        /// Channel description
        description: Option<String>,
        /// Custom icon attachment
        icon: Option<Attachment>,

        /// Default permissions assigned to users in this channel
        default_permissions: Option<OverrideField>,
        /// Permissions assigned based on role to this channel
        #[serde(default = "HashMap::<String, OverrideField>::new")]
        role_permissions: HashMap<String, OverrideField>,

        /// Whether this channel is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },
}

/// Partial values of [Channel]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PartialChannel {
    /// Display name of the channel
    pub name: Option<String>,
    /// User id of the owner of the group
    pub owner: Option<String>,
    /// Channel description
    pub description: Option<String>,
    /// Custom icon attachment
    pub icon: Option<Attachment>,
    /// Whether this channel is marked as not safe for work
    pub nsfw: Option<bool>,
    /// Whether this direct message channel is currently open on both sides
    pub active: Option<bool>,
    /// Permissions assigned to members of this channel
    pub permissions: Option<i64>,
    /// Permissions assigned based on role to this channel
    pub role_permissions: Option<HashMap<String, OverrideField>>,
    /// Default permissions assigned to users in this channel
    pub default_permissions: Option<OverrideField>,
    /// Id of the last message sent in this channel
    pub last_message_id: Option<String>,
}

/// Optional fields on channel object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsChannel {
    Description,
    Icon,
    DefaultPermissions,
}
