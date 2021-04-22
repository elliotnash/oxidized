use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod message;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
