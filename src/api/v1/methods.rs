//! This module contains the Mastodon API Methods `/api/v1`.
//!
pub struct ApiMethod {
    pub endpoint: String,
    pub method: AccountsMethod
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

pub enum AppsMethod {
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
