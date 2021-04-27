use oxidized::{
    client::ClientBuilder,
    event::EventHandler,
    models::{
        message::Message,
        context::Context
    },
    async_trait
};

struct Events;
#[async_trait]
impl EventHandler for Events {
    async fn on_message(&self, ctx: Context, event: Message) {
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
        .event_handler(Events)
        .login().await
        .expect("Failed to create client")
        .run().await;
    
}
