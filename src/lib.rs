use tracing::{info, debug};
use reqwest::StatusCode;
use async_tungstenite::tungstenite::{Message, handshake::client::Request};
use futures::prelude::*;

mod http;
use http::models::connection::Credentials;

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";

pub struct Client {
    http_client: reqwest::Client
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Self {
        let http_client = reqwest::ClientBuilder::new()
            .cookie_store(true).build().unwrap();

        let result = http_client.post(format!("{}/login", BASE_URL))
            .json(&Credentials{email: email.to_string(), password: password.to_string()}).send().await;
        
        let response = match result {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        info!("Logged in to guilded.gg!");
                    }
                    StatusCode::BAD_REQUEST => {
                        panic!("Invalid login credentials");
                    }
                    code => {
                        panic!("Connection to guilded returned: {}", code);
                    }
                };
                response
            }
            Err(error) => {
                debug!("Connection error:\n{}", error);
                if let Some(status) = error.status() {
                    panic!("Connection to guilded returned: {}", status);
                } else if error.is_timeout() {
                    panic!("Connection to guilded timed out");
                } else {
                    panic!("Error connecting to guilded");
                }
            }
        };

        let cookies: Vec<reqwest::cookie::Cookie> = response.cookies().collect();
        let auth_token = cookies[0].value();

        let (mut stream, resp) = async_tungstenite::tokio::connect_async_with_config(
            Request::builder().uri(WS_URL).header("cookie", auth_token).body(()).unwrap(),
            Some(async_tungstenite::tungstenite::protocol::WebSocketConfig{
                accept_unmasked_frames: false,
                max_message_size: None,
                max_frame_size: None,
                max_send_queue: None,
            }),
        ).await.unwrap();

        dbg!(resp);

        let text = "Hello, World!";
        println!("Sending: \"{}\"", text);
        stream.send(Message::text(text)).await.unwrap();

        let msg = stream.next().await.ok_or("didn't receive anything").unwrap().unwrap();
        println!("Received: {:?}", msg);

        Client{http_client}
    }

    pub async fn connect(&self) {
        info!("Connected to guilded.gg");
    }
}