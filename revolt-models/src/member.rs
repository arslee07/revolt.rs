use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use crate::attachment::Attachment;

/// Composite primary key consisting of server and user id
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MemberCompositeKey {
    /// Server Id
    pub server: String,
    /// User Id
    pub user: String,
}

/// Representation of a member of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Member {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,

    /// Time at which this user joined the server
    pub joined_at: Timestamp,

    /// Member's nickname
    pub nickname: Option<String>,
    /// Avatar attachment
    pub avatar: Option<Attachment>,

    /// Member's roles
    #[serde(default)]
    pub roles: Vec<String>,
    /// Timestamp this member is timed out until
    pub timeout: Option<Timestamp>,
}

/// Partial representation of a member of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct PartialMember {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: Option<MemberCompositeKey>,

    /// Time at which this user joined the server
    pub joined_at: Option<Timestamp>,

    /// Member's nickname
    pub nickname: Option<String>,
    /// Avatar attachment
    pub avatar: Option<Attachment>,

    /// Member's roles
    pub roles: Option<Vec<String>>,
    /// Timestamp this member is timed out until
    pub timeout: Option<Timestamp>,
}

/// Optional fields on server member object
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsMember {
    Nickname,
    Avatar,
    Roles,
    Timeout,
}
