use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    #[doc = "Permission"]
    pub struct Permission: u64 {
        // * Generic permissions
        /// Manage the channel or channels on the server
        const ManageChannel = 1 << 0;
        /// Manage the server
        const ManageServer = 1 << 1;
        /// Manage permissions on servers or channels
        const ManagePermissions = 1 << 2;
        /// Manage roles on server
        const ManageRole = 1 << 3;
        /// Manage server customisation (includes emoji)
        const ManageCustomisation = 1 << 4;

        // % 1 bits reserved

        // * Member permissions
        /// Kick other members below their ranking
        const KickMembers = 1 << 6;
        /// Ban other members below their ranking
        const BanMembers = 1 << 7;
        /// Timeout other members below their ranking
        const TimeoutMembers = 1 << 8;
        /// Assign roles to members below their ranking
        const AssignRoles = 1 << 9;
        /// Change own nickname
        const ChangeNickname = 1 << 10;
        /// Change or remove other's nicknames below their ranking
        const ManageNicknames = 1 << 11;
        /// Change own avatar
        const ChangeAvatar = 1 << 12;
        /// Remove other's avatars below their ranking
        const RemoveAvatars = 1 << 13;

        // % 7 bits reserved

        // * Channel permissions
        /// View a channel
        const ViewChannel = 1 << 20;
        /// Read a channel's past message history
        const ReadMessageHistory = 1 << 21;
        /// Send a message in a channel
        const SendMessage = 1 << 22;
        /// Delete messages in a channel
        const ManageMessages = 1 << 23;
        /// Manage webhook entries on a channel
        const ManageWebhooks = 1 << 24;
        /// Create invites to this channel
        const InviteOthers = 1 << 25;
        /// Send embedded content in this channel
        const SendEmbeds = 1 << 26;
        /// Send attachments and media in this channel
        const UploadFiles = 1 << 27;
        /// Masquerade messages using custom nickname and avatar
        const Masquerade = 1 << 28;
        /// React to messages with emojis
        const React = 1 << 29;

        // * Voice permissions
        /// Connect to a voice channel
        const Connect = 1 << 30;
        /// Speak in a voice call
        const Speak = 1 << 31;
        /// Share video in a voice call
        const Video = 1 << 32;
        /// Mute other members with lower ranking in a voice call
        const MuteMembers = 1 << 33;
        /// Deafen other members with lower ranking in a voice call
        const DeafenMembers = 1 << 34;
        /// Move members between voice channels
        const MoveMembers = 1 << 35;

        // * Misc. permissions
        // % Bits 36 to 52: free area
        // % Bits 53 to 64: do not use
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    #[doc = "User permission"]
    pub struct UserPermission: u64 {
        const Access = 1 << 0;
        const ViewProfile = 1 << 1;
        const SendMessage = 1 << 2;
        const Invite = 1 << 3;
    }
}

/// Representation of a single permission override
/// as it appears on models and in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverrideField {
    /// Allow bit flags
    a: Permission,
    /// Disallow bit flags
    d: Permission,
}

/// Representation of a single permission override
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Override {
    /// Allow bit flags
    allow: Permission,
    /// Disallow bit flags
    deny: Permission,
}
