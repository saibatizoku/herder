pub mod entities;
pub mod methods;

pub use self::entities::{
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
};

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
