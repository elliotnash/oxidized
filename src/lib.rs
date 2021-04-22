use tracing::{info, debug};

mod http;
use http::HttpClient;
pub mod models;
use models::{ClientUser, Credentials};

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";

pub struct Client {
    pub http: HttpClient,
    pub client_user: ClientUser,
    pub credentials: Credentials
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Self {
        let cred = Credentials{email: email.to_string(), password: password.to_string()};
        let (http, client_user) = HttpClient::login(&cred).await;
        let client = Client{http, client_user, credentials: cred};
        info!("Logged in to guilded.gg!");
        client
    }
    async fn reconnect(&mut self) {
        let (http, client_user) = HttpClient::login(&self.credentials).await;
        self.http = http;
        self.client_user = client_user;
        info!("Reconnected to guilded.gg!");
    }

    pub async fn run(&mut self) {
        self.http.run().await;
    }

}