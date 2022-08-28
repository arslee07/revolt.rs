use serde::Serialize;

use crate::{
    channel::FieldsChannel,
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
    pub name: Option<String>,
    /// Channel description
    pub description: Option<String>,
    /// Group owner
    pub owner: Option<String>,
    /// Icon attachment ID
    pub icon: Option<String>,
    /// Whether this channel is age-restricted
    pub nsfw: Option<bool>,
    /// Fields to remove
    pub remove: Option<Vec<FieldsChannel>>,
}
