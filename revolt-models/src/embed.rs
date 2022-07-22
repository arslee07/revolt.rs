use crate::attachment::Attachment;
use serde::Deserialize;

/// Embed
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    Video(Video),
    Text(Text),
    None,
}

/// Image
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Image {
    /// URL to the original image
    pub url: String,

    /// Width of the image
    pub width: isize,

    /// Height of the image
    pub height: isize,

    /// Positioning and size
    pub size: ImageSize,
}

/// Image positioning and size
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImageSize {
    /// Show large preview at the bottom of the embed
    Large,

    /// Show small preview to the side of the embed
    Preview,
}

/// Video
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Video {
    /// URL to the original video
    pub url: String,

    /// Width of the video
    pub width: isize,

    /// Height of the video
    pub height: isize,
}

/// Text Embed
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Text {
    /// URL to icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    /// URL for title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Title of text embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Description of text embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of uploaded attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Attachment>,

    /// CSS colour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

/// Information about special remote content
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum Special {
    /// No remote content
    None,

    /// Content hint that this contains a GIF
    ///
    /// Use metadata to find video or image to play
    GIF,

    /// YouTube video
    YouTube {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<String>,
    },

    /// Lightspeed.tv stream
    Lightspeed {
        content_type: LightspeedType,
        id: String,
    },

    /// Twitch stream or clip
    Twitch {
        content_type: TwitchType,
        id: String,
    },
    /// Spotify track
    Spotify { content_type: String, id: String },

    /// Soundcloud track
    Soundcloud,

    /// Bandcamp track
    Bandcamp {
        content_type: BandcampType,
        id: String,
    },
}

/// Type of remote Twitch content
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}

/// Type of remote Lightspeed.tv content
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LightspeedType {
    Channel,
}

/// Type of remote Bandcamp content
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BandcampType {
    Album,
    Track,
}

/// Website metadata
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    /// Direct URL to web page
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    /// Original direct URL
    #[serde(skip_serializing_if = "Option::is_none")]
    original_url: Option<String>,

    /// Remote content
    #[serde(skip_serializing_if = "Option::is_none")]
    special: Option<Special>,

    /// Title of website
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// Description of website
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Embedded image
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,

    /// Embedded video
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Video>,

    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    site_name: Option<String>,

    /// URL to site icon
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,

    /// CSS colour
    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
}
