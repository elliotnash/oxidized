
mod http;
pub mod models;
pub mod error;
pub mod client;
pub mod event;

pub use async_trait::async_trait;

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";
