use oxidized::{
    client::ClientBuilder,
    event::EventHandler,
    async_trait
};

struct Events;
#[async_trait]
impl EventHandler for Events {
    async fn on_message() {
        tracing::info!("RECIEVED MESSAGE EVENT");
    }
}

#[tokio::main]
async fn main() {
    let email = std::env::var("EMAIL").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    tracing_subscriber::fmt::init();
    
    ClientBuilder::new()
        .credentials(&email, &password)
        .login().await
        .unwrap()
        .run().await;
    
}
