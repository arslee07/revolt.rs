use serde::Deserialize;

/// Voice server authentication response
#[derive(Deserialize, Debug, Clone)]
pub struct VoiceAuthenticationData {
    /// Token for authenticating with the voice server
    pub token: String,
}
