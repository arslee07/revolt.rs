pub mod attachment;
pub mod authentication;
pub mod channel;
pub mod core;
pub mod embed;
pub mod emoji;
pub mod event;
pub mod member;
pub mod message;
pub mod payload;
pub mod permission;
pub mod server;
pub mod user;
pub mod voice;

mod error;
pub use error::ApiError;
