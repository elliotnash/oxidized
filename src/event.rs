use std::{fmt::Debug, sync::Arc};
use serde_json::Value;
use tracing::debug;
use crate::models::{
    EventType,
    message::ChatMessageCreated
};


use crate::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message(&self) {}
}
pub(crate) struct DefaultHandler;
impl EventHandler for DefaultHandler{}

pub(crate) struct EventDispatcher {
    pub handler: Arc<dyn EventHandler>
}
impl Debug for EventDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handler: Arc<dyn EventHandler>")
    }
}
impl EventDispatcher {
    pub(crate) async fn event_handler(&self, event_type: EventType, event: Value) {
        match event_type {
            EventType::ChatMessageCreated => {
                debug!("{}", event.to_string());
                let event = serde_json::from_value::<ChatMessageCreated>(event).unwrap();
                debug!("{:?}", event);
            }
        }
    }
}
