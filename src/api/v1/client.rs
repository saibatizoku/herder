//! This module contains the code representing Mastodon nodes and API Clients
//!
use api::APIMethodRequest;
use errors::*;
use hyper::Uri;
use hyper::header::Bearer;
use mastodon::ApiHandler;
use std::str::FromStr;
use super::entities;
use super::methods;
use super::methods::{
    AccountID,
    HomeTimelineQuery,
    NotificationID,
    SearchAccountsQuery,
    SearchContentsQuery,
    StatusID,
    TagID,
    TimelineQuery,
    UserFormData
};
use url::Url;


/// The API Client, currently works for version 1 of the Mastodon API.
pub struct Client {
    pub url_base: Url,
    pub token: Bearer
}

impl ApiHandler for Client {
    fn endpoint_url(&self, path: &str) -> Result<Url> {
        self.url_base.clone().join(path).chain_err(|| "could not join path with URL")
    }
    fn endpoint_url_string(&self, path: &str) -> String {
        self.endpoint_url(path).unwrap().into_string()
    }
}

impl methods::Accounts for Client {
    fn fetch_account(&self, account_id: AccountID) -> Result<entities::Account> {
        let api_method = APIMethodRequest {
            uri: Uri::from_str(&format!("/api/v1/accounts/{}", account_id.id)).unwrap(),
            ..APIMethodRequest::default()
        };
        println!("FETCH ACCOUNT: {:?}", api_method);
        Ok(entities::Account::default())
    }
    fn get_current_user(&self) -> Result<entities::Account> {
        unimplemented!();
    }
    fn update_current_user(&self, form_data: UserFormData) -> Result<entities::Account> {
        unimplemented!()
    }
    fn get_account_followers(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn get_account_following(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn get_account_statutes(&self, account_id: AccountID) -> Result<Vec<entities::Status>> {
        unimplemented!()
    }
    fn follow_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        unimplemented!()
    }
    fn unfollow_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        unimplemented!()
    }
    fn block_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn unblock_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn mute_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        unimplemented!()
    }
    fn unmute_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        unimplemented!()
    }
    fn get_account_relationships(&self, account_id: AccountID) -> Result<Vec<entities::Relationship>> {
        unimplemented!()
    }
    fn search_accounts(&self, query: SearchAccountsQuery) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
}
impl methods::Blocks for Client {
    fn fetch_blocks(&self, query: String) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
}
impl methods::Favourites for Client {
    fn fetch_favourites(&self, query: String) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
}
impl methods::FollowRequests for Client {
    fn fetch_follow_requests(&self, query: String) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn authorize_follow_request(&self, form_data: String) -> Result<()> {
        unimplemented!()
    }
    fn reject_follow_request(&self, form_data: String) -> Result<()> {
        unimplemented!()
    }
}

impl methods::Follows for Client {
    fn follow_remote_user(&self, form_data: String) -> Result<entities::Account> {
        unimplemented!()
    }
}

impl methods::Instances for Client {
    fn get_instance(&self) -> Result<entities::Instance> {
        unimplemented!()
    }
}

impl methods::Media for Client {
    fn upload_media(&self, form_data: String) -> Result<entities::Attachment> {
        unimplemented!()
    }
}

impl methods::Mutes for Client {
    fn fetch_mutes(&self, query: String) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
}

impl methods::Notifications for Client {
    fn fetch_notifications(&self, query: String) -> Result<Vec<entities::Notification>> {
        unimplemented!()
    }
    fn get_notification(&self, notification_id: NotificationID) -> Result<entities::Notification> {
        unimplemented!()
    }
    fn clear_notifications(&self) -> Result<()> {
        unimplemented!()
    }
}

impl methods::Reports for Client {
    fn fetching_reports(&self) -> Result<Vec<entities::Report>> {
        unimplemented!()
    }
    fn reporting_user(&self, form_data: String) -> Result<entities::Report> {
        unimplemented!()
    }
}

impl methods::Search for Client {
    fn search_content(&self, query: SearchContentsQuery) -> Result<entities::Results> {
        unimplemented!()
    }
}

impl methods::Statuses for Client {
    fn fetch_status(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
    fn get_status_context(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
    fn get_status_card(&self, status_id: StatusID) -> Result<entities::Card> {
        unimplemented!()
    }
    fn reblogged_by(&self, status_id: StatusID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn favourited_by(&self, status_id: StatusID) -> Result<Vec<entities::Account>> {
        unimplemented!()
    }
    fn post_status(&self, form_data: String) -> Result<entities::Status> {
        unimplemented!()
    }
    fn delete_status(&self, status_id: StatusID) -> Result<()> {
        unimplemented!()
    }
    fn reblog_status(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
    fn unreblog_status(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
    fn favourite_status(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
    fn unfavourite_status(&self, status_id: StatusID) -> Result<entities::Status> {
        unimplemented!()
    }
}

impl methods::Timelines for Client {
    fn home_timeline(&self, query: HomeTimelineQuery) -> Result<Vec<entities::Status>> {
        unimplemented!()
    }
    fn public_timeline(&self, query: TimelineQuery) -> Result<Vec<entities::Status>> {
        unimplemented!()
    }
    fn tag_timeline(&self, hashtag: TagID, query: TimelineQuery) -> Result<Vec<entities::Status>> {
        unimplemented!()
    }
}
