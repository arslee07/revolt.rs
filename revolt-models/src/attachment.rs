use serde::Deserialize;

/// Metadata associated with attachment
#[derive(Deserialize, Debug, Clone)]
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
#[derive(Deserialize, Debug, Clone)]
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
    pub deleted: Option<bool>,

    /// Whether this attachment was reported
    pub reported: Option<bool>,

    // NOTE: Theese 3 fields will be deprecated in the next update
    pub message_id: Option<String>,
    pub user_id: Option<String>,
    pub server_id: Option<String>,

    /// ID of the object this attachment is associated with
    pub object_id: Option<String>,
}
