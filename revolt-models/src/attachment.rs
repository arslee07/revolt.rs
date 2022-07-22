use serde::Deserialize;

/// Metadata associated with attachment
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum Metadata {
    /// Attachment is just a generic uncategorised file
    File,

    /// Attachment contains textual data and should be displayed as such
    Text,

    /// Attachment is an image with specific dimensions
    Image { width: isize, height: isize },

    /// Attachment is a video with specific dimensions
    Video { width: isize, height: isize },

    /// Attachment is audio
    Audio,
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata::File
    }
}

/// Representation of an attachment on Revolt
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Attachment {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,

    /// Tag/bucket this attachment was uploaded to
    pub tag: String,

    /// Original filename
    pub filename: String,

    /// Parsed metadata of this attachment
    pub metadata: Metadata,

    /// Raw content type of this attachment
    pub content_type: String,

    /// Size of this attachment (in bytes)
    pub size: isize,

    /// Whether this attachment was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Whether this attachment was reported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported: Option<bool>,

    // NOTE: Theese 3 fields will be deprecated in the next update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,

    /// ID of the object this attachment is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
