use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SendMessagePayload {
    pub content: String,
}
