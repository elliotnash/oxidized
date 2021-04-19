use tracing::{info, debug};

mod http;
use http::{HttpClient, models::ClientUser};

const BASE_URL: &str = "https://www.guilded.gg/api";
const WS_URL: &str = "wss://api.guilded.gg/socket.io/?jwt=undefined&EIO=3&transport=websocket";

pub struct Client {
    pub http: HttpClient,
    pub client_user: ClientUser
}

impl Client {

    pub async fn login(email: &str, password: &str) -> Self {
        let (http, client_user) = HttpClient::login(email, password).await;
        let client = Client{http, client_user};
        info!("Logged in to guilded.gg!");
        client
    }

    pub async fn run(&self) {
        self.http.run().await;
    }

}