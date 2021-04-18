use lazy_static::lazy_static;
use tracing::{info, debug};
use reqwest::{Client, StatusCode};

mod http;
use http::models::connection::Credentials;

const BASE_URL: &str = "https://www.guilded.gg/api";

lazy_static!{
    pub static ref HTTP_CLIENT: Client = reqwest::ClientBuilder::new()
        .cookie_store(true).build().unwrap();
}

pub async fn login(email: &str, password: &str){

    let result = HTTP_CLIENT.post(format!("{}/login", BASE_URL))
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
}
