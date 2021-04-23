use oxidized::client::ClientBuilder;

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
