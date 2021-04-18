use tracing::{info, debug};

mod http;
use http::HttpClient;

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";

pub struct Client {
    http: HttpClient
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Self {
        Client{http: HttpClient::login(email, password).await}
    }

    pub async fn run(&self) {
        self.http.run().await;
    }

}