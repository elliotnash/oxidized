
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    oxidized::connect("", "").await;
}
