use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Email {
    #[serde(rename = "in-reply-to")]
    pub in_reply_to: String,
    pub private: bool,
    pub attachments: Vec<String>,
    pub subject: String,
    pub mid: String,
    pub epoch: i64,
    pub list: String,
    pub gravatar: String,
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub from: String,
    pub list_raw: String,
    pub id: String,
    pub body: String,
    #[serde(default)]
    pub date: String,
}
