use std::{fmt::{Debug, write}, sync::Arc};

use crate::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message(&self) {}
}

pub(crate) struct EventDispatcher{
    pub handler: Arc<dyn EventHandler>
}
impl Debug for EventDispatcher{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handler: Arc<dyn EventHandler>")
    }
}
