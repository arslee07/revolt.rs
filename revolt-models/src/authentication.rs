#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Authentication {
    SessionToken(String),
    BotToken(String),
    ValidMfaTicket(String),
    UnvalidatedMfaTicket(String),
}

impl Authentication {
    pub fn header_key(&self) -> String {
        match self {
            Authentication::SessionToken(_) => "x-session-token",
            Authentication::BotToken(_) => "x-bot-token",
            Authentication::ValidMfaTicket(_) => "x-mfa-ticket",
            Authentication::UnvalidatedMfaTicket(_) => "x-mfa-ticket",
        }
        .to_string()
    }

    pub fn value(&self) -> String {
        match self {
            Authentication::SessionToken(t) => t,
            Authentication::BotToken(t) => t,
            Authentication::ValidMfaTicket(t) => t,
            Authentication::UnvalidatedMfaTicket(t) => t,
        }
        .to_string()
    }
}
