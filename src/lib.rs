use tracing::{info, debug};
use reqwest::StatusCode;

mod http;
use http::models::connection::Credentials;

const BASE_URL: &str = "https://www.guilded.gg/api";

pub async fn connect(email: &str, password: &str){

    let client = reqwest::Client::new();

    let result = client.post(format!("{}/login", BASE_URL))
        .json(&Credentials{email: email.to_string(), password: password.to_string()}).send().await;
    
    let something = match result {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    info!("Connect to guilded.gg!");
                }
                StatusCode::BAD_REQUEST => {
                    panic!("Invalid login credentials");
                }
                _ => {}
            }
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
    dbg!(something);
}
