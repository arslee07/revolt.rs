use crate::permission::{Permission, UserPermission};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum ApiError {
    LabelMe,

    // Onboarding related errors
    AlreadyOnboarded,

    // User related errors
    UsernameTaken,
    InvalidUsername,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,

    // Channel related errors
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments,
    TooManyReplies,
    EmptyMessage,
    PayloadTooLarge,
    CannotRemoveYourself,
    GroupTooLarge { max: usize },
    AlreadyInGroup,
    NotInGroup,

    // Server related errors
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers { max: usize },
    TooManyEmoji,

    // Bot related errors
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,

    // Permission errors
    MissingPermission { permission: Permission },
    MissingUserPermission { permission: UserPermission },
    NotElevated,
    CannotGiveMissingPermissions,
    NotOwner,

    // General errors
    DatabaseError { operation: String, with: String },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidSession,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,
    NoEffect,
    FailedValidation,

    // Other errors that API does not return but it's still API related things
    Unauthenticated,
}
