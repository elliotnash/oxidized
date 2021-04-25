//! <img src="https://raw.githubusercontent.com/elliotnash/oxidized/main/logo.svg" alt="drawing" width="200"/>
//! 
//! A Guilded.gg API wrapper for Rust
//! 
//! Oxidized relies on Guilded's private api, so features are unstable
//! and could break at any time. We hope to update Oxidized to use the 
//! public bot api once it's released
//! 
//! Oxidized is in it's early stages, and currently does not cover
//! the entireity of the guilded api. Contributions are welcome,
//! checkout our [github](https://github.com/elliotnash/oxidized).
//! 
//! # Getting Started
//! 
//! ### Installation
//! Start by adding the following to your dependancies section
//! of your `Cargo.toml`:
//! ```toml
//! oxidized = "0.1"
//! ```
//! To compile from git, use the following:
//! ```toml
//! oxidized = { git = "https://github.com/elliotnash/oxidized.git", branch = "main" }
//! ```
//! You will also need the tokio runtime:
//! ```toml
//! [dependencies.tokio]
//! version = "1"
//! features = ["macros", "rt-multi-thread"]
//! ```
//! 
//! ### Basic Usage
//! Create a new [`ClientBuilder`](`client::ClientBuilder`) to configure your client.
//! Loging in is preformed with user credentials (email password), as Guilded's bot api
//! has not been released yet. We suggest creating a new account for your bot to 
//! run under, and not using your main account.
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     ClientBuilder::new()
//!         .credentials("email", "password")
//!         .login().await
//!         .expect("Failed to create client")
//!         .run().await;
//! }
//! ```
//! 
//! ### Event Handling
//! If you are familliar with Serenity, you'll notice that events in Oxidized are
//! handled in a very similar way. To recieve events, you'll need to pass a struct
//! to your [`ClientBuilder`](`client::ClientBuilder`) that implements 
//! [`EventHandler`](`event::EventHandler`).
//! ```rust
//! struct Events;
//! #[async_trait]
//! impl EventHandler for Events {
//!     async fn on_message(&self, event: ChatMessageCreated) {
//!         // fired when a message is recieved
//!     }
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     ClientBuilder::new()
//!         .credentials("email", "password")
//!         .event_handler(Events)
//!         .login().await
//!         .expect("Failed to create client")
//!         .run().await;
//! }
//! ```
//! To see the full list of events, look at [`event::EventHandler`].
//! Remember all methods need to be async.
//! 
//! ### Examples
//! To see full examples, checkout our [github](https://github.com/elliotnash/oxidized/tree/main/examples)
pub mod http;
pub mod models;
pub mod error;
pub mod client;
pub mod event;

pub use async_trait::async_trait;

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";
