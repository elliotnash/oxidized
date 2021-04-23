use crate::async_trait;

#[async_trait]
pub trait EventHandler{
    async fn on_message() {}
}
