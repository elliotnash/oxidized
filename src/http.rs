use serde_json::Value;
use std::sync::Arc;
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
use regex::Regex;
use lazy_static::lazy_static;
use crate::{BASE_URL, WS_URL};

use crate::models::{ClientUser, ClientUserRoot, message::ChatMessageCreated, Credentials, EventType, Hello};


type WsStream = Arc<RwLock<SplitStream<WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>>>>;
type WsSink = Arc<RwLock<SplitSink<WebSocketStream<Stream<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>, Message>>>;


pub struct HttpClient {
    http_client: reqwest::Client,
    ws_stream: WsStream,
    ws_sink: WsSink
}

impl HttpClient {

    pub(crate) async fn login(cred: &Credentials) -> (Self, ClientUser) {
        let http_client = reqwest::ClientBuilder::new()
            .cookie_store(true).build().unwrap();

        let result = http_client.post(format!("{}/login", BASE_URL))
            .json(&cred).send().await;
        
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
        (HttpClient{http_client, ws_stream: Arc::new(RwLock::new(ws_stream)), ws_sink: Arc::new(RwLock::new(ws_sink))}, client_user)

    }

    pub(crate) async fn run(&self) {
        let hello_text = &self.ws_stream.write().await.next().await
            .expect("Invalid acknowledgement packet")
            .expect("Invalid acknowledgement packet").into_text()
            .expect("Invalid acknowledgement packet")[1..];
        let hello = serde_json::from_str::<Hello>(hello_text).expect("Invalid acknowledgement packet");
        info!("Connected to guilded.gg");

        let event_listener_task = tokio::spawn(Self::event_listener(self.ws_stream.clone(), hello.clone()));
        let heartbeat_task = tokio::spawn(Self::heartbeat(self.ws_sink.clone(), hello.clone()));
        if let Err(err) = event_listener_task.await {
            debug!("Error in task heartbeat_task: {}", err);
        }
        heartbeat_task.abort();
        info!("Disconnected from guilded.gg!");
    }

    async fn heartbeat(ws_sink: WsSink, hello: Hello) {
        debug!("{:?}", &hello);
        loop{
            if let Err(err) = ws_sink.write().await.send(Message::text("2")).await {
                debug!("Couldn't ping websocket: {:?}", err);
                break;
            }
            sleep(Duration::from_millis(hello.ping_interval as u64)).await;
        }
    }

    async fn event_listener(ws_stream: WsStream, hello: Hello) {
        loop{
            match tokio::time::timeout(Duration::from_millis((hello.ping_timeout+hello.ping_interval) as u64), async {
                if let Some(Ok(Message::Text(msg))) = ws_stream.write().await.next().await {
                    let rm = RawMessage::from_raw(&msg);
                    match rm.code {
                        3 => {
                            debug!("Server heartbeat received");
                        },
                        42 => {
                            if let Ok((event_type, event)) = serde_json::from_str::<(EventType, Value)>(&rm.json) {
                                Self::event_handler(event_type, event).await;
                            } else {
                                info!("Received unkown event, message: {}", rm.json)
                            }
                        },
                        _ => info!("Received unkown code: {}, message: {}", rm.code, rm.json)
                    }
                }
            }).await {
                Ok(_) => {}
                Err(_) => {
                    debug!("Websocket didn't recieve heartbeat within timeout!");
                    break;
                }
            }
        }
    }

    async fn event_handler(event_type: EventType, event: Value) {
        match event_type {
            EventType::ChatMessageCreated => {
                debug!("{}", event.to_string());
                let event = serde_json::from_value::<ChatMessageCreated>(event).unwrap();
                debug!("{:?}", event);
            }
        }
    }

}


struct RawMessage {
    pub code: i32,
    pub json: String
}
impl RawMessage {
    fn from_raw(raw_string: &str) -> Self {
        lazy_static! {
            static ref RMRE: Regex = Regex::new("^[0-9]*").unwrap();
        }
        let (code, end_index) = RMRE.find(raw_string).map_or((-1, 0), 
            |m| (m.as_str().parse::<i32>().unwrap_or(-1), m.end()));
        
        RawMessage{code, json: (&raw_string[end_index..]).to_string()}
    }
}
