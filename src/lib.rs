use tracing::{info, debug};
use reqwest::StatusCode;

mod http;
use http::models::connection::Credentials;

const BASE_URL: &str = "https://www.guilded.gg/api";

pub struct Client {
    client: reqwest::Client
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Self {
        let client = Client{
            client: reqwest::ClientBuilder::new()
                .cookie_store(true).build().unwrap()
        };

        let result = client.client.post(format!("{}/login", BASE_URL))
            .json(&Credentials{email: email.to_string(), password: password.to_string()}).send().await;
        
        match result {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        info!("Logged in to guilded.gg!");
                    }
                    StatusCode::BAD_REQUEST => {
                        panic!("Invalid login credentials");
                    }
                    _ => {}
                };
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
        client
    }

    pub async fn connect(&self) {
        info!("Connected to guilded.gg");
    }
}