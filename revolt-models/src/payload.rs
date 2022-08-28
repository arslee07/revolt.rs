use serde::Serialize;

use crate::{
    channel::FieldsChannel,
    permission::Override,
    user::{FieldsUser, PartialUserProfile, UserStatus},
};

#[derive(Serialize, Debug, Clone)]
pub struct SendMessagePayload {
    pub content: String,
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
        pub permissions: Permission,
    },
    Field {
        /// Allow / deny values to set for members in this [Channels::TextChannel] or [Channels::VoiceChannel]
        pub permissions: Override,
    },
}
