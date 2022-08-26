use serde::Deserialize;

/// Information about what owns this emoji
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: String },
    Detached,
}

/// Representation of an Emoji on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Emoji {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// What owns this emoji
    pub parent: EmojiParent,
    /// Uploader user id
    pub creator_id: String,
    /// Emoji name
    pub name: String,
    /// Whether the emoji is animated
    #[serde(default)]
    pub animated: bool,
    /// Whether the emoji is marked as nsfw
    #[serde(default)]
    pub nsfw: bool,
}
