use std::sync::Arc;
use crate::http::HttpClient;

#[derive(Debug)]
pub struct Context{
    pub http: Arc<HttpClient>
}

impl Context {
}
