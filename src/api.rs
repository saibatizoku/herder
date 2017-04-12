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

    pub struct Account {
        pub id: i64,
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

    pub struct Application {
        pub name: String,
        pub website: Option<String>
    }

    pub struct Attachment {
        pub id: i64,
        pub attachment_type: String,
        pub url: String,
        pub remote_url: String,
        pub preview_url: String,
        pub text_url: String

    }

    pub struct Card {
        pub url: String,
        pub title: String,
        pub descrption: String,
        pub image: String,
    }

    pub struct Context {
        pub ancestors: Vec<Status>
    }

    pub struct Error {
    }

    pub struct Instance {
    }

    pub struct Mention {
    }

    pub struct Notification {
    }

    pub struct Relationship {
    }

    pub struct Results {
    }

    pub struct Status {
    }

    pub struct Tag {
    }
}
