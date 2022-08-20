use serde::Deserialize;

/// Revolt instance configuration
#[derive(Deserialize, Debug, Clone)]
pub struct InstanceConfiguration {
    /// Revolt API version
    pub revolt: String,

    /// Features enabled on this Revolt node
    pub features: InstanceFeatures,

    /// WebSocket URL
    pub ws: String,

    /// URL pointing to the client serving this node
    pub app: String,

    /// Web Push VAPID public key
    pub vapid: String,
}

/// Features enabled on this Revolt node
#[derive(Deserialize, Debug, Clone)]
pub struct InstanceFeatures {
    /// hCaptcha configuration
    pub captcha: CaptchaConfiguration,

    /// Whether email verification is enabled
    pub email: bool,

    /// Whether this instance is invite only
    pub invite_only: bool,

    /// File server service configuration
    pub autumn: AutumnConfiguration,

    /// Proxy server configuration
    pub january: JanuaryConfiguration,

    /// Voice server configuration
    pub voso: VosoConfiguration,
}

/// hCaptcha configuration
#[derive(Deserialize, Debug, Clone)]
pub struct CaptchaConfiguration {
    /// Whether captcha is enabled
    pub enabled: bool,

    /// Client key used for solving captcha
    pub key: String,
}

/// File server service configuration
#[derive(Deserialize, Debug, Clone)]
pub struct AutumnConfiguration {
    /// Whether the service is enabled
    pub enabled: bool,

    /// URL pointing to this service
    pub url: String,
}

/// Proxy server configuration
#[derive(Deserialize, Debug, Clone)]
pub struct JanuaryConfiguration {
    /// Whether the service is enabled
    pub enabled: bool,

    /// URL pointing to this service
    pub url: String,
}

/// Voice server configuration
#[derive(Deserialize, Debug, Clone)]
pub struct VosoConfiguration {
    /// Whether the service is enabled
    pub enabled: bool,

    /// URL pointing to the voice API
    pub url: String,

    /// URL pointing to the voice WebSocket server
    pub ws: String,
}
