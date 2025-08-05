use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Email {
    #[serde(rename = "in-reply-to")]
    pub in_reply_to: String,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub mid: String,
    #[serde(default)]
    pub epoch: i64,
    #[serde(default)]
    pub list: String,
    #[serde(default)]
    pub gravatar: String,
    #[serde(rename = "message-id", default)]
    pub message_id: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub list_raw: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub date: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Attachment {
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub size:  i64,
    #[serde(default)]
    pub hash: String,
}
