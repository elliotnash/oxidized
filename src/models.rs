use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientUserRoot {
    pub user: ClientUser
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientUser {
    pub id: String,
    pub name: String,
    pub subdomain: Option<String>,
    pub aliases: Vec<Alias>,
    pub profile_picture_sm: Option<String>,
    pub profile_picture: Option<String>,
    pub profile_picture_lg: Option<String>,
    pub profile_picture_blur: Option<String>,
    pub profile_banner_blur: Option<String>,
    pub profile_banner_lg: Option<String>,
    pub join_date: DateTime<Utc>,
    pub steam_id: Option<String>,
    pub moderation_status: Option<String>,
    pub about_info: Option<String>,
    pub last_online: DateTime<Utc>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    game_id: i32,
    name: String,
    social_link_source: Option<String>,
    additional_info: Value,
    edited_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hello {
    pub sid: String,
    pub upgrades: Vec<String>,
    pub ping_interval: i32,
    pub ping_timeout: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    ChatMessageCreated
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageCreated {
    pub channel_category_id: Option<i32>,
    pub channel_id: String,
    //TODO channel_type enum
    pub channel_type: String,
    pub content_id: String,
    //TODO content_type enum
    pub content_type: String,
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
    //TODO make object type enums
    object: String,
    data: Option<GenericData>,
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
    //TODO make object type enums
    id: i32,
    custom_reaction_id: i32
}
