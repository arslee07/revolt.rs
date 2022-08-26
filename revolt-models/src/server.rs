use std::collections::HashMap;

use serde::Deserialize;

use crate::{attachment::Attachment, permission::OverrideField};

/// Representation of a server role
#[derive(Deserialize, Debug, Clone)]
pub struct Role {
    /// Role name
    pub name: String,
    /// Permissions available to this role
    pub permissions: OverrideField,
    /// Colour used for this role
    ///
    /// This can be any valid CSS colour
    pub colour: Option<String>,
    /// Whether this role should be shown separately on the member sidebar
    #[serde(default)]
    pub hoist: bool,
    /// Ranking of this role
    #[serde(default)]
    pub rank: i64,
}

/// Partial representation of a server role
#[derive(Deserialize, Debug, Clone)]
pub struct PartialRole {
    /// Role name
    pub name: Option<String>,
    /// Permissions available to this role
    pub permissions: Option<OverrideField>,
    /// Colour used for this role
    ///
    /// This can be any valid CSS colour
    pub colour: Option<String>,
    /// Whether this role should be shown separately on the member sidebar
    pub hoist: Option<bool>,
    /// Ranking of this role
    pub rank: Option<i64>,
}

/// Channel category
#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    /// Unique ID for this category
    pub id: String,
    /// Title for this category
    pub title: String,
    /// Channels in this category
    pub channels: Vec<String>,
}

/// System message channel assignments
#[derive(Deserialize, Debug, Clone)]
pub struct SystemMessageChannels {
    /// ID of channel to send user join messages in
    pub user_joined: Option<String>,
    /// ID of channel to send user left messages in
    pub user_left: Option<String>,
    /// ID of channel to send user kicked messages in
    pub user_kicked: Option<String>,
    /// ID of channel to send user banned messages in
    pub user_banned: Option<String>,
}

bitflags::bitflags! {
    /// Server flag enum
    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct ServerFlags: u64 {
        const Verified = 1;
        const Official = 2;
    }
}

/// Representation of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// User id of the owner
    pub owner: String,

    /// Name of the server
    pub name: String,
    /// Description for the server
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Vec<String>,
    /// Categories for this server
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    #[serde(default = "HashMap::<String, Role>::new")]
    pub roles: HashMap<String, Role>,
    /// Default set of server and channel permissions
    pub default_permissions: i64,

    /// Icon attachment
    pub icon: Option<Attachment>,
    /// Banner attachment
    pub banner: Option<Attachment>,

    /// Enum of server flags
    pub flags: Option<i32>,

    /// Whether this server is flagged as not safe for work
    #[serde(default)]
    pub nsfw: bool,
    /// Whether to enable analytics
    #[serde(default)]
    pub analytics: bool,
    /// Whether this server should be publicly discoverable
    #[serde(default)]
    pub discoverable: bool,
}

/// Partial representation of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct PartialServer {
    /// User id of the owner
    pub owner: Option<String>,

    /// Name of the server
    pub name: Option<String>,
    /// Description for the server
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Option<Vec<String>>,
    /// Categories for this server
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    pub roles: Option<HashMap<String, Role>>,
    /// Default set of server and channel permissions
    pub default_permissions: Option<i64>,

    /// Icon attachment
    pub icon: Option<Attachment>,
    /// Banner attachment
    pub banner: Option<Attachment>,

    /// Enum of server flags
    pub flags: Option<i32>,

    /// Whether this server is flagged as not safe for work
    pub nsfw: Option<bool>,
    /// Whether to enable analytics
    pub analytics: Option<bool>,
    /// Whether this server should be publicly discoverable
    pub discoverable: Option<bool>,
}

/// Optional fields on server object
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}

/// Optional fields on server object
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsRole {
    Colour,
}
