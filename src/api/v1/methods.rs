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

/// Methods for interacting with accounts on a Mastodon node.
pub trait Accounts {
    /// Fetching an account:
    ///
    /// ```
    /// GET /api/v1/accounts/:account_id
    /// ```
    ///
    /// Returns an `Account`.
    fn fetch_account(&self, account_id: usize) -> Result<entities::Account, &str>;

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
    fn get_account_followers(&self, account_id: usize) -> Result<Vec<entities::Account>, &str>;

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
    fn get_account_following(&self, account_id: usize) -> Result<Vec<entities::Account>, &str>;

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
    fn get_account_statutes(&self, account_id: usize) -> Result<Vec<entities::Status>, &str>;

    /// Following an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/follow
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn follow_account(&self, account_id: usize) -> Result<entities::Relationship, &str>;

    /// Unfollowing an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unfollow
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unfollow_account(&self, account_id: usize) -> Result<entities::Relationship, &str>;

    /// Blocking an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/block
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn block_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str>;

    /// Unblocking an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unblock
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unblock_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str>;

    /// Muting an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/mute
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn mute_account(&self, account_id: usize) -> Result<entities::Relationship, &str>;

    /// Unmuting an account:
    ///
    /// ```
    /// POST /api/v1/accounts/:account_id/unmute
    /// ```
    ///
    /// Returns the target account's `Relationship`.
    fn unmute_account(&self, account_id: usize) -> Result<entities::Relationship, &str>;

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
    fn get_account_relationships(&self, account_id: usize) -> Result<Vec<entities::Relationship>, &str>;

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
    fn search_accounts(&self, query: String) -> Result<Vec<entities::Account>, &str>;
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
}

pub trait Instances {
}

pub trait Media {
}

pub trait Mutes {
}

pub trait Notifications {
}

pub trait Reports {
}

pub trait Search {
}

pub trait Statuses {
}

pub trait Timelines {
}
