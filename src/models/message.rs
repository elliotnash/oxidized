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
impl ChatMessageCreated {
    pub fn fill_message(mut self) -> Self {
        self.message.team_id = Some(self.team_id.clone());
        self.message.channel_id = Some(self.channel_id.clone());
        self.message.channel_category_id = self.channel_category_id.clone();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub bot_id: Option<String>,
    pub content: MessageContent,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub team_id: Option<String>,
    pub channel_id: Option<String>,
    pub channel_category_id: Option<i32>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    document: Document
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    nodes: Vec<Node>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "object")]
pub enum Node {
    Block(Block),
    Text(Text),
    Inline(Inline)
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    leaves: Vec<Leaf>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    data: BlockData,
    #[serde(rename = "type")]
    block_type: BlockType,
    nodes: Vec<Node>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct BlockData {
    embeds: Vec<Embed>
}
impl Default for BlockData {
    fn default() -> Self {
        Self{embeds: Vec::new()}
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BlockType {
    Paragraph,
    #[serde(rename = "webhookMessage")]
    WebhookMessage,
    #[serde(rename = "systemMessage")]
    SystemMessage,
    ListItem,
    UnorderedList,
    Image,
    CodeLine,
    CodeContainer,
    Form
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Inline {
    #[serde()]
    Reaction(ReactionData),
    Channel(Channel)
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    data: ChannelData,
    nodes: Vec<Node>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelData {
    channel: ChannelObject
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelObject {
    id: String,
    matcher: String,
    name: String
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    data: ReactionData
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReactionData {
    reaction: ReactionObject
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReactionObject {
    id: i32,
    custom_reaction_id: i32
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Leaf {
    text: String,
    marks: Vec<Mark>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MarkType {
    InlineCodeV2,
    Bold,
    Italic
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mark {
    #[serde(rename = "type")]
    mark_type: MarkType
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    author: EmbedAuthor,
    color: i32,
    description: String,
    title: String,
    url: String
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmbedAuthor {
    icon_url: String,
    name: String,
    url: String
}
