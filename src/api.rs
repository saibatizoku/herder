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
        id: i32,
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
        url: String,
        title: String,
        description: String,
        email: String
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Mention {
        id: i32,
        url: String,
        username: String,
        acct: String
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Notification {
        id: i32,
        notification_type: String,
        created_at: String,
        account: Account,
        status: Option<Status>
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Relationship {
        id: i32,
        following: bool,
        followed_by: bool,
        blocking: bool,
        muting: bool,
        requested: bool
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Report {
        id: i32,
        action_taken: String,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Results {
        accounts: Vec<Account>,
        statuses: Vec<Status>,
        hashtags: Vec<String>
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status {
        id: i32,
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
        name: String,
        url: String
    }
}
