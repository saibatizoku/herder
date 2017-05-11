//! This module contains the Mastodon API Entities `/api/v1`.
//!
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Account {
    #[serde(rename = "id")]
    pub account_id: i32,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub header: String,
    pub locked: bool,
    pub created_at: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub statuses_count: i32
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Application {
    pub name: String,
    pub website: Option<String>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Attachment {
    #[serde(rename = "id")]
    pub attachment_id: i32,
    #[serde(rename = "type")]
        pub attachment_type: String,
        pub url: String,
        pub remote_url: String,
        pub preview_url: String,
        pub text_url: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Error {
    pub error: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Instance {
    pub url: String,
    pub title: String,
    pub description: String,
    pub email: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Mention {
    #[serde(rename = "id")]
    pub mention_id: i32,
    pub url: String,
    pub username: String,
    pub acct: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Notification {
    #[serde( rename = "id")]
    pub notification_id: i32,
    #[serde( rename = "type")]
        pub notification_type: String,
        pub created_at: String,
        pub account: Account,
        pub status: Option<Status>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Relationship {
    #[serde(rename = "id")]
    pub relationship_id: i32,
    pub following: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub muting: bool,
    pub requested: bool
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Report {
    #[serde(rename = "id")]
    pub report_id: i32,
    pub action_taken: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Status {
    #[serde(rename = "id")]
    pub status_id: i32,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub in_reply_to_id: Option<i32>,
    pub in_reply_to_account_id: Option<i32>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub created_at: String,
    pub reblogs_count: i32,
    pub favourites_count: i32,
    pub reblogged: bool,
    pub favourited: bool,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub application: Application
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String
}
