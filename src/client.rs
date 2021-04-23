use std::sync::Arc;

use tokio::time::{Duration, sleep};
use crate::{
    error::{
        LoginError,
        LoginErrorType
    },
    event::{
        EventHandler,
        EventDispatcher
    }
};
use tracing::info;

use crate::http::HttpClient;
use crate::models::{ClientUser, Credentials};

#[derive(Debug)]
pub struct Client {
    pub http: HttpClient,
    pub client_user: ClientUser,
    pub credentials: Credentials
}

impl Client {
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

pub struct ClientBuilder {
    credentials: Option<Credentials>,
    event_handler: Option<Arc<dyn EventHandler>>
}

impl ClientBuilder{
    pub fn new() -> Self {
        Self {
            credentials: None,
            event_handler: None
        }
    }
    pub fn credentials(&mut self, email: &str, password: &str) -> &mut Self {
        self.credentials = Some(Credentials{
            email: email.to_string(), password: password.to_string()
        });
        self
    }
    pub fn event_handler<H: EventHandler + 'static>(&mut self, event_handler: H) -> &mut Self {
        self.event_handler = Some(Arc::new(event_handler));
        self
    }
    pub async fn login(&self) -> Result<Client, LoginError> {
        let cred = self.credentials.clone().ok_or(LoginError{error_type: LoginErrorType::ConnectionError})?;
        let (mut http, client_user) = HttpClient::login(&cred).await?;
        if let Some(handler) = self.event_handler.clone() {
            http.dispatcher = EventDispatcher{handler};
        }
        let client = Client{http, client_user, credentials: cred};
        info!("Logged in to guilded.gg!");
        Ok(client)
    }
}
