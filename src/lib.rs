use tokio::time::{Duration, sleep};
use error::LoginError;
use tracing::info;

mod http;
use http::HttpClient;
pub mod models;
use models::{ClientUser, Credentials};
pub mod error;

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";

pub struct Client {
    pub http: HttpClient,
    pub client_user: ClientUser,
    pub credentials: Credentials
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Result<Self, LoginError> {
        let cred = Credentials{email: email.to_string(), password: password.to_string()};
        let (http, client_user) = HttpClient::login(&cred).await?;
        let client = Client{http, client_user, credentials: cred};
        info!("Logged in to guilded.gg!");
        Ok(client)
    }
    async fn reconnect(&mut self) {
        loop {
            sleep(Duration::from_secs(10)).await;
            info!("Attempting to reconnect to guilded.gg");
            if let Ok((http, client_user)) = HttpClient::login(&self.credentials).await {
                self.http = http;
                self.client_user = client_user;
                break;
            }
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.http.run().await;
            self.reconnect().await;
        }
    }

}