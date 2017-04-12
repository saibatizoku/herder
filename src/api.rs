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

    pub struct Application {
        name: String,
        website: Option<String>
    }

    pub struct Attachment {
        id: i32,
        attachment_type: String,
        url: String,
        remote_url: String,
        preview_url: String,
        text_url: String

    }

    pub struct Card {
        url: String,
        title: String,
        descrption: String,
        image: String,
    }

    pub struct Context {
        ancestors: Vec<Status>,
        descendants: Vec<Status>
    }

    pub struct Error {
        error: String
    }

    pub struct Instance {
        url: String,
        title: String,
        description: String,
        email: String
    }

    pub struct Mention {
        id: i32,
        url: String,
        username: String,
        acct: String
    }

    pub struct Notification {
        id: i32,
        notification_type: String,
        created_at: String,
        account: Account,
        status: Option<Status>
    }

    pub struct Relationship {
        id: i32,
        following: bool,
        followed_by: bool,
        blocking: bool,
        muting: bool,
        requested: bool
    }

    pub struct Report {
        id: i32,
        action_taken: String,
    }

    pub struct Results {
        accounts: Vec<Account>,
        statuses: Vec<Status>,
        hashtags: Vec<String>
    }

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

    pub struct Tag {
        name: String,
        url: String
    }
}
