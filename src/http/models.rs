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
    pub aliases: Vec<String>,
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
    pub channel_category_id: i32,
    pub channel_id: String,
    //TODO channel_type enum
    pub channel_type: String,
    pub content_id: String,
    //TODO content_type enum
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    //TODO userID struct
    pub created_by: String,
    pub guilded_client_id: String,
    //TODO message struct
    pub message: Value,
    pub team_id: String
}
