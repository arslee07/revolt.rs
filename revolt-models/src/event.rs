use serde::{Deserialize, Serialize};

use crate::{
    channel::{Channel, FieldsChannel, PartialChannel},
    emoji::Emoji,
    member::{FieldsMember, Member, MemberCompositeKey, PartialMember},
    message::{AppendMessage, Message, PartialMessage},
    server::{FieldsRole, FieldsServer, PartialRole, PartialServer, Server},
    user::{FieldsUser, PartialUser, RelationshipStatus, User, UserSettings},
};

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

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    /// Multiple events
    Bulk { v: Vec<ServerToClientEvent> },

    /// Error
    Error { error: ErrorId },

    /// Successfully authenticated
    Authenticated,

    /// Basic data to cache
    Ready {
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>,
        members: Vec<Member>,
        emojis: Option<Vec<Emoji>>,
    },

    /// Ping response
    Pong { data: u32 },

    /// New message
    Message {
        #[serde(flatten)]
        message: Message,
    },

    /// Update existing message
    MessageUpdate {
        id: String,
        channel: String,
        data: PartialMessage,
    },

    /// Append information to existing message
    MessageAppend {
        id: String,
        channel: String,
        append: AppendMessage,
    },

    /// Delete message
    MessageDelete { id: String, channel: String },

    /// New reaction to a message
    MessageReact {
        id: String,
        channel_id: String,
        user_id: String,
        emoji_id: String,
    },

    /// Remove user's reaction from message
    MessageUnreact {
        id: String,
        channel_id: String,
        user_id: String,
        emoji_id: String,
    },

    /// Remove a reaction from message
    MessageRemoveReaction {
        id: String,
        channel_id: String,
        emoji_id: String,
    },

    /// Bulk delete messages
    BulkMessageDelete { channel: String, ids: Vec<String> },

    /// New channel
    ChannelCreate(Channel),

    /// Update existing channel
    ChannelUpdate {
        id: String,
        data: PartialChannel,
        clear: Vec<FieldsChannel>,
    },

    /// Delete channel
    ChannelDelete { id: String },

    /// User joins a group
    ChannelGroupJoin { id: String, user: String },

    /// User leaves a group
    ChannelGroupLeave { id: String, user: String },

    /// User started typing in a channel
    ChannelStartTyping { id: String, user: String },

    /// User stopped typing in a channel
    ChannelStopTyping { id: String, user: String },

    /// User acknowledged message in channel
    ChannelAck {
        id: String,
        user: String,
        message_id: String,
    },

    /// New server
    ServerCreate {
        id: String,
        server: Server,
        channels: Vec<Channel>,
    },

    /// Update existing server
    ServerUpdate {
        id: String,
        data: PartialServer,
        clear: Vec<FieldsServer>,
    },

    /// Delete server
    ServerDelete { id: String },

    /// Update existing server member
    ServerMemberUpdate {
        id: MemberCompositeKey,
        data: PartialMember,
        clear: Vec<FieldsMember>,
    },

    /// User joins server
    ServerMemberJoin { id: String, user: String },

    /// User left server
    ServerMemberLeave { id: String, user: String },

    /// Server role created or updated
    ServerRoleUpdate {
        id: String,
        role_id: String,
        data: PartialRole,
        clear: Vec<FieldsRole>,
    },

    /// Server role deleted
    ServerRoleDelete { id: String, role_id: String },

    /// Update existing user
    UserUpdate {
        id: String,
        data: PartialUser,
        clear: Vec<FieldsUser>,
    },

    /// Relationship with another user changed
    UserRelationship {
        id: String,
        user: User,
        // ! this field can be deprecated
        status: RelationshipStatus,
    },

    /// Settings updated remotely
    UserSettingsUpdate { id: String, update: UserSettings },

    /// New emoji
    EmojiCreate(Emoji),

    /// Delete emoji
    EmojiDelete { id: String },
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
