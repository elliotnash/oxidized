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
use crate::{BASE_URL, WS_URL};

pub mod models;
use models::{
    Credentials,
    ClientUser,
    ClientUserRoot
};



pub struct HttpClient {
    http_client: reqwest::Client,
    ws_stream: RwLock<SplitStream<WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>>>,
    ws_sink: RwLock<SplitSink<WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>, Message>>
}

impl HttpClient {

    pub(crate) async fn login(email: &str, password: &str) -> (Self, ClientUser) {
        let http_client = reqwest::ClientBuilder::new()
            .cookie_store(true).build().unwrap();

        let result = http_client.post(format!("{}/login", BASE_URL))
            .json(&Credentials{email: email.to_string(), password: password.to_string()}).send().await;
        
        let response = match result {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        debug!("Received response from /api/login");
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

        let cookies = response.headers().get("Set-Cookie").expect("Invalid response from server").clone();
        let client_user = response.json::<ClientUserRoot>().await.expect("Invalid response from server").user;

        let (ws_stream, _) = async_tungstenite::tokio::connect_async_with_config(
            Request::builder().uri(WS_URL).header("cookie", cookies).body(()).unwrap(),
            Some(async_tungstenite::tungstenite::protocol::WebSocketConfig{
                accept_unmasked_frames: false,
                max_message_size: None,
                max_frame_size: None,
                max_send_queue: None,
            }),
        ).await.unwrap();

        let (ws_sink, ws_stream) = ws_stream.split();
        (HttpClient{http_client, ws_stream: RwLock::new(ws_stream), ws_sink: RwLock::new(ws_sink)}, client_user)

    }

    pub(crate) async fn run(&self) {
        info!("Connected to guilded.gg");
        let both = future::join(self.heartbeat(), self.event_handler());
        both.await;
    }

    async fn heartbeat(&self) {
        loop{
            self.ws_sink.write().await.send(Message::text("2")).await.unwrap();
            sleep(Duration::from_millis(25000)).await;
        }
    }

    async fn event_handler(&self) {
        loop{
            if let Some(Ok(Message::Text(msg))) = self.ws_stream.write().await.next().await {
                if msg == "3" {
                    debug!("Server heartbeat received");
                } else {
                    info!("Received: {:?}", msg);
                }
            }
        }
    }

}