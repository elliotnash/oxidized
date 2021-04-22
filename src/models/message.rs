use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::channel::{ChannelType, ContentType};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageCreated {
    pub channel_category_id: Option<i32>,
    //TODO channel_id struct
    pub channel_id: String,
    pub channel_type: ChannelType,
    //TODO content_id struct
    pub content_id: String,
    pub content_type: ContentType,
    pub created_at: DateTime<Utc>,
    //TODO userID struct
    pub created_by: String,
    pub guilded_client_id: Option<String>,
    pub message: Message,
    pub team_id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    content: MessageContent,
    created_at: DateTime<Utc>,
    created_by: String,
    id: String,
    r#type: String
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    object: String,
    document: Document
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    object: String,
    data: Option<GenericData>,
    nodes: Vec<GenericNode>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericNode {
    object: String,
    data: Option<GenericData>,
    leaves: Option<Vec<Leaf>>,
    nodes: Option<Vec<GenericNode>>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericData {
    reaction: Option<Reaction>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    id: i32,
    custom_reaction_id: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Leaf {
    object: String,
    text: String,
    marks: Vec<String>
}
