use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Credentials {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ClientUserRoot {
    pub user: ClientUser
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientUser {
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
pub(crate) struct Hello {
    pub sid: String,
    pub upgrades: Vec<String>,
    pub ping_interval: i32,
    pub ping_timeout: i32
}
