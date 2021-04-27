use std::{fmt::Debug, sync::Arc};
use serde_json::error::Error;
use serde_json::Value;
use crate::{http::HttpClient, models::{EventType, context::Context, message::ChatMessageCreated}};


use crate::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message(&self, _ctx: Context, _event: ChatMessageCreated) {}
}
pub(crate) struct DefaultHandler;
impl EventHandler for DefaultHandler{}

#[derive(Clone)]
pub(crate) struct EventDispatcher {
    pub handler: Arc<dyn EventHandler>
}
impl Debug for EventDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{handler: Arc<dyn EventHandler>}}")
    }
}
impl EventDispatcher {
    pub(crate) async fn event_handler(&self, http: Arc<HttpClient>, event_type: EventType, event: Value) {
        let dispatcher = self.clone();
        tokio::spawn(async move {
            dispatcher.dispatcher(http, event_type, event).await
        });
    }
    async fn dispatcher(&self, http: Arc<HttpClient>, event_type: EventType, event: Value) -> Result<(), Error> {
        match event_type {
            EventType::ChatMessageCreated => {
                let event = serde_json::from_value::<ChatMessageCreated>(event)?.fill_message();
                let ctx = Context{http};
                self.handler.on_message(ctx, event).await;
                Ok(())
            }
        }
    }
}
