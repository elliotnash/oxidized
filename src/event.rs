use crate::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message(&self) {}
}
