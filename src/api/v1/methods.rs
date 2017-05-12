//! This module contains the Mastodon API Methods `/api/v1`.
//!

use api::oauth::OAuthApp;
use super::entities;

/// updatable fields for the authenticated user.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct UserFormData {
    display_name: Option<String>,
    note: Option<String>,
    avatar: Option<String>,
    header: Option<String>
}

impl UserFormData {
    pub fn new(&self, display_name: Option<String>, note: Option<String>, avatar: Option<String>, header: Option<String>) -> Self {
        UserFormData {
            display_name,
            note,
            avatar,
            header
        }
    }
}

/// Account ID.
#[derive(Debug, PartialEq)]
pub struct AccountID {
    pub id: usize
}

/// Notification ID.
#[derive(Debug, PartialEq)]
pub struct NotificationID {
    pub id: usize
}

/// Status ID.
#[derive(Debug, PartialEq)]
pub struct StatusID {
    pub id: usize
}

/// Tag ID.
#[derive(Debug, PartialEq)]
pub struct TagID {
    pub id: usize
}

/// Fields to query accounts.
pub struct SearchAccountsQuery {
    pub q: String,
    pub limit: Option<usize>
}

/// Fields to query contents.
pub struct SearchContentsQuery {
    pub q: String,
    pub resolve: Option<bool>
}

/// Fields to query the home timeline.
pub struct HomeTimelineQuery {
    pub max_id: Option<usize>,
    pub since_id: Option<usize>,
    pub limit: Option<usize>
}

/// Fields to query the public/tag timeline.
pub struct TimelineQuery {
    pub local: Option<bool>,
    pub max_id: Option<usize>,
    pub since_id: Option<usize>,
    pub limit: Option<usize>
}

/// Methods for interacting with accounts on a Mastodon node.
pub trait Accounts {
    /// Fetching an account:
    ///
    /// ```
    /// GET /api/v1/accounts/:account_id
    /// ```
    ///
    /// Returns an `Account`.
    fn fetch_account(&self, account_id: AccountID) -> Result<entities::Account, &str>;

    /// Getting the current user:
    ///
    /// ```
    /// GET /api/v1/accounts/verify_credentials
    /// ```
    ///
    /// Returns the authenticated user's `Account`.
    fn get_current_user(&self) -> Result<entities::Account, &str>;

    /// Updating the current user:
    ///
    /// ```
    /// PATCH /api/v1/accounts/update_credentials
    /// ```
    ///
    /// `UserFormData` form data is required.
    fn update_current_user(&self, form_data: UserFormData) -> Result<entities::Account, &str>;

    /// Getting an account's followers:
    ///
    /// ```
    /// GET /api/v1/accounts/:account_id/followers
    /// ```
    /// Query parameters:
    ///
    /// `max_id` Get a list of followers with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of followers with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of followers to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s.
    fn get_account_followers(&self, account_id: AccountID) -> Result<Vec<entities::Account>, &str>;

    /// Get who account is following:
    ///
    /// ```
    /// GET /api/v1/accounts/:account_id/following
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id` Get a list of followers with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of followers with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of followers to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s.
    fn get_account_following(&self, account_id: AccountID) -> Result<Vec<entities::Account>, &str>;

    /// Get an account's statuses:
    ///
    /// ```
    /// GET /api/v1/accounts/:account_id/statuses
    /// ```
    ///
    /// Query parameters:
    ///
    /// `only_media` Only return statuses that have media attachments. It is optional.
    ///
    /// `exclude_replies` Skip statuses that reply to other statuses. It is optional.
    ///
    /// `max_id` Get a list of statutes with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of statutes with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of statutes to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Status`es.
    fn get_account_statutes(&self, account_id: AccountID) -> Result<Vec<entities::Status>, &str>;

    /// Following an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/follow
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn follow_account(&self, account_id: AccountID) -> Result<entities::Relationship, &str>;

    /// Unfollowing an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unfollow
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unfollow_account(&self, account_id: AccountID) -> Result<entities::Relationship, &str>;

    /// Blocking an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/block
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn block_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>, &str>;

    /// Unblocking an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unblock
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unblock_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>, &str>;

    /// Muting an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/mute
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn mute_account(&self, account_id: AccountID) -> Result<entities::Relationship, &str>;

    /// Unmuting an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unmute
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unmute_account(&self, account_id: AccountID) -> Result<entities::Relationship, &str>;

    /// Getting an account's relationships:
    ///
    /// ```
    /// GET /api/v1/accounts/relationships
    /// ```
    ///
    /// Query parameters:
    ///
    /// `id` Account IDs (can be an array). It is required.
    ///
    /// Returns an array of `Relationship`s of the current user to a list of given accounts.
    fn get_account_relationships(&self, account_id: AccountID) -> Result<Vec<entities::Relationship>, &str>;

    /// Searching for accounts:
    ///
    /// ```
    /// GET /api/v1/accounts/search
    /// ```
    ///
    /// Query parameters:
    ///
    /// `q` What to search for. It is required.
    ///
    /// `limit` Maximum number of matching accounts to return (default: `40`). It is optional.
    ///
    /// Returns an array of matching `Account`s.
    ///
    /// Will lookup an account remotely if the search term is in the `username@domain` format and
    /// not yet in the database.
    fn search_accounts(&self, query: SearchAccountsQuery) -> Result<Vec<entities::Account>, &str>;
}

pub trait Apps {
    /// Registering an application:
    ///
    /// ```
    /// POST /api/v1/apps
    /// ```
    ///
    /// Form data:
    ///
    /// `client_name` Name of your application. It is required.
    ///
    /// `redirect_uris` Where the user should be redirected after authorization (for no redirect,
    ///                 use `urn:ietf:wg:oauth:2.0:oob`. It is required.
    ///
    /// `scopes` This can be space-separated list of the following items "read", "write" and
    ///          "follow". It is required.
    ///
    /// `website` URL to the homepage of your app. It is optional.
    ///
    /// Creates and returns a new `OAuthApp`.
    fn register_app(&self, name: &str, uris: &str, scopes: &str) -> OAuthApp;
}

pub trait Blocks {
    /// Fetching a user's blocks:
    ///
    /// ```
    /// GET /api/v1/blocks
    /// ```
    /// Query parameters:
    ///
    /// `max_id` Get a list of blocks with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of blocks with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of blocks to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s blocked by the authenticated user.
    fn fetch_blocks(&self, query: String) -> Result<Vec<entities::Account>, &str>;
}
pub trait Favourites {
    /// Fetching a user's favourites:
    ///
    /// ```
    /// GET /api/v1/favourites
    /// ```
    /// Query parameters:
    ///
    /// `max_id` Get a list of favourites with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of favourites with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of favourites to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s favourited by the authenticated user.
    fn fetch_favourites(&self, query: String) -> Result<Vec<entities::Account>, &str>;
}

pub trait FollowRequests {
    /// Fetching a list of follow requests:
    ///
    /// ```
    /// GET /api/v1/follow_requests
    /// ```
    /// Query parameters:
    ///
    /// `max_id` Get a list of follow requests with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of follow requests with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of follow requests to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s which have requested to follow the authenticated user.
    fn fetch_follow_requests(&self, query: String) -> Result<Vec<entities::Account>, &str>;

    /// Authorizing follow requests
    ///
    /// ```
    /// POST /api/v1/follow_requests/:account_id/authorize
    /// ```
    ///
    /// Parameters:
    ///
    /// `id` The id of the account to authorize. It is required.
    ///
    /// Returns an empty object.
    fn authorize_follow_request(&self, form_data: String) -> Result<(), &str>;

    /// Rejecting follow requests
    ///
    /// ```
    /// POST /api/v1/follow_requests/:account_id/reject
    /// ```
    ///
    /// Parameters:
    ///
    /// `id` The id of the account to authorize. It is required.
    ///
    /// Returns an empty object.
    fn reject_follow_request(&self, form_data: String) -> Result<(), &str>;
}

pub trait Follows {
    /// Following a remote user:
    ///
    /// ```
    /// POST /api/v1/follows
    /// ```
    ///
    /// Form data:
    ///
    /// `uri` `username@domain` of the person you want to follow. It is required.
    ///
    /// Returns the local representation of the followed `Account`.
    fn follow_remote_user(&self, form_data: String) -> Result<entities::Account, &str>;
}

pub trait Instances {
    /// Getting instance information:
    ///
    /// ```
    /// GET /api/v1/instance
    /// ```
    ///
    /// Returns the current `Instance`.
    ///
    /// Does not require authentication.
    fn get_instance(&self) -> Result<entities::Instance, &str>;
}

pub trait Media {
    /// Uploading a media attachment:
    ///
    /// ```
    /// POST /api/v1/media
    /// ```
    ///
    /// Form data:
    ///
    /// `file` Media to be uploaded. It is required.
    ///
    /// Returns an `Attachment`that can be used when creating a status.
    fn upload_media(&self, form_data: String) -> Result<entities::Attachment, &str>;
}

pub trait Mutes {
    /// Fetching a user's mutes
    ///
    /// ```
    /// GET /api/v1/mutes
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id` Get a list of mutes with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of mutes with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of mutes to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Account`s muted by the authenticated user.
    fn fetch_mutes(&self, query: String) -> Result<Vec<entities::Account>, &str>;
}

pub trait Notifications {
    /// Fetching a user's notifications:
    ///
    /// ```
    /// GET /api/v1/notifications
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id` Get a list of notifications with ID less than or equal this value. It is optional.
    ///
    /// `since_id` Get a list of notifications with ID greater than this value. It is optional.
    ///
    /// `limit` Maximum number of notifications to get (Default 40, Max 80). It is optional.
    ///
    /// Returns an array of `Notification`s for the authenticated user.
    fn fetch_notifications(&self, query: String) -> Result<Vec<entities::Notification>, &str>;

    /// Getting a single notification:
    ///
    /// ```
    /// GET /api/v1/notifications/:notification_id
    /// ```
    ///
    /// Returns the `Notification`
    fn get_notification(&self, notification_id: NotificationID) -> Result<entities::Notification, &str>;

    /// Clearing notifications:
    ///
    /// ```
    /// POST /api/v1/notifications/clear
    /// ```
    ///
    /// Deletes all notifications from the Mastodon server for the authenticated user. Returns an
    /// empty object.
    fn clear_notifications(&self) -> Result<(), &str>;
}

pub trait Reports {
    /// Fetching a user's reports:
    ///
    /// ```
    /// GET /api/v1/reports
    /// ```
    ///
    /// Returns a list of `Report`s made by the authenticated user..
    fn fetching_reports(&self) -> Result<Vec<entities::Report>, &str>;

    /// Reporting a user:
    ///
    /// ```
    /// POST /api/v1/reports
    /// ```
    ///
    /// Form data:
    ///
    /// `account_id` The ID of the account to report. It is required.
    ///
    /// `status_ids` The IDs of the status to report (can be an array). It is required.
    ///
    /// `comment` A comment to associate with the report. It is required.
    ///
    /// Returns the finished `Report`.
    fn reporting_user(&self, form_data: String) -> Result<entities::Report, &str>;
}

pub trait Search {
    /// Searching for content:
    ///
    /// ```
    /// GET /api/v1/search
    /// ```
    ///
    /// Form data:
    ///
    /// `q` The search query. It is required.
    ///
    /// `resolve` Whether to resolve non-local accounts. It is required.
    ///
    /// Returns `Results`.
    ///
    /// If `q` is a URL, Mastodon will attempt to fetch the provided account or status. Otherwise,
    /// it will do a local account and hashtag search.
    ///
    /// Does not require authentication.
    fn search_content(&self, query: SearchContentsQuery) -> Result<entities::Results, &str>;
}

pub trait Statuses {
    /// Fetching a status:
    ///
    /// ```
    /// GET /api/v1/statuses/:status_id
    /// ```
    ///
    /// Returns a `Status`.
    ///
    /// Does not require authentication.
    fn fetch_status(&self, status_id: StatusID) -> Result<entities::Status, &str>;

    /// Getting status context:
    ///
    /// ```
    /// GET /api/v1/statuses/:status_id/context
    /// ```
    ///
    /// Returns a `Context`.
    ///
    /// Does not require authentication.
    fn get_status_context(&self, status_id: StatusID) -> Result<entities::Status, &str>;

    /// Getting a card associated with a status:
    ///
    /// ```
    /// GET /api/v1/statuses/:status_id/card
    /// ```
    ///
    /// Returns a `Card`.
    ///
    /// Does not require authentication.
    fn get_status_card(&self, status_id: StatusID) -> Result<entities::Card, &str>;


    /// Getting who reblogged a status:
    ///
    /// ```
    /// GET /api/v1/statuses/:status_id/reblogged_by
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id`  Get a list of reblogged with ID less than or equal this value    yes
    ///
    /// `since_id`    Get a list of reblogged with ID greater than this value  yes
    ///
    /// `limit`   Maximum number of reblogged to get (Default 40, Max 80)  yes
    ///
    /// Returns an array of `Account`s.
    ///
    /// Does not require authentication.
    fn reblogged_by(&self, status_id: StatusID) -> Result<Vec<entities::Account>, &str>;

    /// Getting who favourited a status:
    ///
    /// ```
    /// GET /api/v1/statuses/:status_id/favourited_by
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id`  Get a list of favourited with ID less than or equal this value    yes
    ///
    /// `since_id`    Get a list of favourited with ID greater than this value  yes
    ///
    /// `limit`   Maximum number of favourited to get (Default 40, Max 80)  yes
    ///
    /// Returns an array of `Account`s.
    ///
    /// Does not require authentication.
    fn favourited_by(&self, status_id: StatusID) -> Result<Vec<entities::Account>, &str>;

    /// Posting a new status:
    ///
    /// ```
    /// POST /api/v1/statuses
    /// ```
    ///
    /// Form data:
    ///
    /// `status`  The text of the status  no
    /// `in_reply_to_id`  local ID of the status you want to reply to. It is optional.
    /// `media_ids`   Array of media IDs to attach to the status (maximum 4). It is optional.    /// `sensitive`   Set this to mark the media of the status as NSFW. It is optional.
    /// `spoiler_text`    Text to be shown as a warning before the actual content. It is optional.
    /// `visibility`  Either "direct", "private", "unlisted" or "public". It is optional.
    ///
    /// Returns the new `Status`.
    fn post_status(&self, form_data: String) -> Result<entities::Status, &str>;

    /// Deleting a status:
    ///
    /// ```
    /// DELETE /api/v1/statuses/:status_id
    /// ```
    ///
    /// Returns an empty object.
    fn delete_status(&self, status_id: StatusID) -> Result<(), &str>;

    /// Reblogging a status:
    ///
    /// ```
    /// POST /api/v1/statuses/:status_id/reblog
    /// ```
    ///
    /// Returns the target Status.
    fn reblog_status(&self, status_id: StatusID) -> Result<entities::Status, &str>;

    /// Unreblogging a status:
    ///
    /// ```
    /// POST /api/v1/statuses/:status_id/unreblog
    /// ```
    ///
    /// Returns the target Status.
    fn unreblog_status(&self, status_id: StatusID) -> Result<entities::Status, &str>;

    /// Favouriting/unfavouriting a status:
    ///
    /// ```
    /// POST /api/v1/statuses/:status_id/favourite
    /// ```
    ///
    /// Returns the target Status.
    fn favourite_status(&self, status_id: StatusID) -> Result<entities::Status, &str>;

    /// Favouriting/unfavouriting a status:
    ///
    /// ```
    /// POST /api/v1/statuses/:status_id/unfavourite
    /// ```
    ///
    /// Returns the target Status.
    fn unfavourite_status(&self, status_id: StatusID) -> Result<entities::Status, &str>;
}

pub trait Timelines {
    /// Retrieving a home timeline:
    ///
    /// ```
    /// GET /api/v1/timelines/home
    /// ```
    ///
    /// Query parameters:
    ///
    /// `max_id`  Get a list of timelines with ID less than or equal this value. It is optional.
    ///
    /// `since_id`    Get a list of timelines with ID greater than this value. It is optional.
    ///
    /// `limit`   Maximum number of statuses on the requested timeline to get (Default 20, Max 40).
    /// It is optional.
    ///
    /// Returns an array of `Status`es, most recent ones first.
    fn home_timeline(&self, query: HomeTimelineQuery) -> Result<Vec<entities::Status>, &str>;

    /// Retrieving a public timeline:
    ///
    /// ```
    /// GET /api/v1/timelines/public
    /// ```
    ///
    /// Query parameters:
    ///
    /// `local`   Only return statuses originating from this instance. It is optional.
    ///
    /// `max_id`  Get a list of timelines with ID less than or equal this value. It is optional.
    ///
    /// `since_id`    Get a list of timelines with ID greater than this value. It is optional.
    ///
    /// `limit`   Maximum number of statuses on the requested timeline to get (Default 20, Max 40).
    /// It is optional.
    ///
    /// Returns an array of `Status`es, most recent ones first.
    ///
    /// Does not require authentication.
    fn public_timeline(&self, query: TimelineQuery) -> Result<Vec<entities::Status>, &str>;

    /// Retrieving a timeline:
    ///
    /// ```
    /// GET /api/v1/timelines/tag/:hashtag
    /// ```
    ///
    /// Query parameters:
    ///
    /// `local`   Only return statuses originating from this instance. It is optional.
    ///
    /// `max_id`  Get a list of timelines with ID less than or equal this value. It is optional.
    ///
    /// `since_id`    Get a list of timelines with ID greater than this value. It is optional.
    ///
    /// `limit`   Maximum number of statuses on the requested timeline to get (Default 20, Max 40).
    /// It is optional.
    ///
    /// Returns an array of `Status`es, most recent ones first.
    ///
    /// Does not require authentication.
    fn tag_timeline(&self, hashtag: TagID, query: TimelineQuery) -> Result<Vec<entities::Status>, &str>;
}
