
#[tokio::main]
async fn main() {
    let email = std::env::var("EMAIL").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    tracing_subscriber::fmt::init();
    oxidized::login(&email, &password).await;
}
