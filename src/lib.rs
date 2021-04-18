use tokio::{
    sync::RwLock,
    net::TcpStream,
    time::{sleep, Duration}
};
use tokio_native_tls::TlsStream;
use tracing::{info, debug};
use reqwest::StatusCode;
use async_tungstenite::{
    WebSocketStream, 
    stream::Stream, 
    tokio::TokioAdapter, 
    tungstenite::{
        Message, 
        handshake::client::Request
    }
};
use futures::{prelude::*, stream::{SplitSink, SplitStream}};

mod http;
use http::{HttpClient, models::connection::Credentials};

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