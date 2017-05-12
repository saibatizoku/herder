//! This module contains the Mastodon API Methods `/api/v1`.
//!

use api::oauth::OAuthApp;
use super::entities;

pub trait Accounts {
    /// Returns an `Account` given its unique `account_id`.
    fn fetch_account(&self, account_id: usize) -> Result<entities::Account, &str>;
    /// gets the `Account` for the authenticated user.
    /// 
    /// `account_id` is required.
    fn get_current_user(&self) -> Result<entities::Account, &str>;
    /// updates the `Account` for the authenticated user.
    ///
    /// `UserFormData` form data is required.
    fn update_current_user(&self, form_data: String) -> Result<entities::Account, &str>;
    /// get a vector of the account's followers.
    fn get_account_followers(&self, account_id: usize) -> Result<Vec<entities::Account>, &str>;
}

pub enum AccountsMethod {
    FetchAccount,
    GetCurrentUser,
    UpdateCurrentUser,
    GetAccountFollowers,
    GetFollowing,
    GetAccountStatuses,
    FollowAccount,
    UnfollowAccount,
    MuteAccount,
    UnmuteAccount,
    GetAccountRelationships,
    SearchAccounts
}

pub trait AppsMethod {
    fn register_app(&self, name: &str, uris: &str, scopes: &str) -> OAuthApp;
}

pub enum BlocksMethod {
}

pub enum FavoritesMethod {
}

pub enum FollowRequestsMethod {
}

pub enum FollowsMethod {
}

pub enum InstancesMethod {
}

pub enum MediaMethod {
}

pub enum MutesMethod {
}

pub enum NotificationsMethod {
}

pub enum ReportsMethod {
}

pub enum SearchMethod {
}

pub enum StatusesMethod {
}

pub enum TimelinesMethod {
}
