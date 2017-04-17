pub mod v1 {
    pub enum Entity {
        Account,
        Application,
        Attachment,
        Card,
        Context,
        Error,
        Instance,
        Mention,
        Notification,
        Relationship,
        Report,
        Results,
        Status,
        Tag
    }

    pub enum Method {
        Accounts,
        Apps,
        Blocks,
        Favorites,
        FollowRequests,
        Follows,
        Instances,
        Media,
        Mutes,
        Notifications,
        Reports,
        Search,
        Statuses,
        Timelines
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Account {
        #[serde(rename = "id")]
        account_id: i32,
        username: String,
        acct: String,
        display_name: String,
        note: String,
        url: String,
        avatar: String,
        header: String,
        locked: bool,
        created_at: String,
        followers_count: i32,
        following_count: i32,
        statuses_count: i32
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
        ancestors: Vec<Status>,
        descendants: Vec<Status>
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
        notification_id: i32,
        #[serde( rename = "type")]
        notification_type: String,
        created_at: String,
        account: Account,
        status: Option<Status>
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
        accounts: Vec<Account>,
        statuses: Vec<Status>,
        hashtags: Vec<String>
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status {
        #[serde(rename = "id")]
        status_id: i32,
        uri: String,
        url: String,
        account: Account,
        in_reply_to_id: Option<i32>,
        in_reply_to_account_id: Option<i32>,
        reblog: Option<Box<Status>>,
        content: String,
        created_at: String,
        reblogs_count: i32,
        favourites_count: i32,
        reblogged: bool,
        favourited: bool,
        sensitive: bool,
        spoiler_text: String,
        visibility: String,
        media_attachments: Vec<Attachment>,
        mentions: Vec<Mention>,
        tags: Vec<Tag>,
        application: Application
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Tag {
        pub name: String,
        pub url: String
    }
}
