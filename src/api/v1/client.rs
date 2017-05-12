//! This module contains the code representing Mastodon nodes and API Clients
//!
use serde_json;
use url::Url;

use api::APIMethod;
use api::oauth::{CreateApp, OAuthApp};
use mastodon::ApiHandler;
use super::entities;
use super::methods;

use std::sync::{Arc, Mutex};


/// The API Client, currently works for version 1 of the Mastodon API.
pub struct Client {
    pub url_base: String,
    pub token: String
}

impl ApiHandler for Client {
    fn endpoint_url_string(&self, path: &str) -> String {
        let base = Url::parse(&self.url_base).unwrap();
        base.join(path).unwrap().into_string()
    }
}

impl methods::Accounts for Client {
    fn fetch_account(&self, account_id: usize) -> Result<entities::Account, &str> {
        let api_method = APIMethod {
            endpoint: format!("/api/v1/accounts/{}", account_id),
            ..APIMethod::default()
        };
        println!("FETCH ACCOUNT: {:?}", api_method);
        Ok(entities::Account::default())
    }
    fn get_current_user(&self) -> Result<entities::Account, &str> {
        unimplemented!();
    }
    fn update_current_user(&self, form_data: methods::UserFormData) -> Result<entities::Account, &str> {
        unimplemented!()
    }
    fn get_account_followers(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn get_account_following(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn get_account_statutes(&self, account_id: usize) -> Result<Vec<entities::Status>, &str> {
        unimplemented!()
    }
    fn follow_account(&self, account_id: usize) -> Result<entities::Relationship, &str> {
        unimplemented!()
    }
    fn unfollow_account(&self, account_id: usize) -> Result<entities::Relationship, &str> {
        unimplemented!()
    }
    fn block_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn unblock_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn mute_account(&self, account_id: usize) -> Result<entities::Relationship, &str> {
        unimplemented!()
    }
    fn unmute_account(&self, account_id: usize) -> Result<entities::Relationship, &str> {
        unimplemented!()
    }
    fn get_account_relationships(&self, account_id: usize) -> Result<Vec<entities::Relationship>, &str> {
        unimplemented!()
    }
    fn search_accounts(&self, search_query: String) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
impl methods::Blocks for Client {
    fn fetch_blocks(&self, query: String) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
impl methods::Favourites for Client {
    fn fetch_favourites(&self, query: String) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
impl methods::FollowRequests for Client {
    fn fetch_follow_requests(&self, query: String) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn authorize_follow_request(&self, form_data: String) -> Result<(), &str> {
        unimplemented!()
    }
    fn reject_follow_request(&self, form_data: String) -> Result<(), &str> {
        unimplemented!()
    }
}

impl methods::Follows for Client {
    fn follow_remote_user(&self, form_data: String) -> Result<entities::Account, &str> {
        unimplemented!()
    }
}

impl methods::Instances for Client {
    fn get_instance(&self) -> Result<entities::Instance, &str> {
        unimplemented!()
    }
}

impl methods::Media for Client {
    fn upload_media(&self, form_data: String) -> Result<entities::Attachment, &str> {
        unimplemented!()
    }
}

impl methods::Mutes for Client {
    fn fetch_mutes(&self, query: String) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}

impl methods::Notifications for Client {
    fn fetch_notifications(&self, query: String) -> Result<Vec<entities::Notification>, &str> {
        unimplemented!()
    }
    fn get_notification(&self, notification_id: usize) -> Result<entities::Notification, &str> {
        unimplemented!()
    }
    fn clear_notifications(&self) -> Result<(), &str> {
        unimplemented!()
    }
}

impl methods::Reports for Client {
    fn fetching_reports(&self) -> Result<Vec<entities::Report>, &str> {
        unimplemented!()
    }
    fn reporting_user(&self, form_data: String) -> Result<entities::Report, &str> {
        unimplemented!()
    }
}

impl methods::Search for Client {
    fn search_content(&self, query: String) -> Result<entities::Results, &str> {
        unimplemented!()
    }
}

impl methods::Statuses for Client {
    fn fetch_status(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn get_status_context(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn get_status_card(&self, status_id: usize) -> Result<entities::Card, &str> {
        unimplemented!()
    }
    fn reblogged_by(&self, status_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn favourited_by(&self, status_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn post_status(&self, form_data: String) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn delete_status(&self, status_id: usize) -> Result<(), &str> {
        unimplemented!()
    }
    fn reblog_status(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn unreblog_status(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn favourite_status(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
    fn unfavourite_status(&self, status_id: usize) -> Result<entities::Status, &str> {
        unimplemented!()
    }
}

impl methods::Timelines for Client {
    fn home_timeline(&self, query: String) -> Result<Vec<entities::Status>, &str> {
        unimplemented!()
    }
    fn public_timeline(&self, query: String) -> Result<Vec<entities::Status>, &str> {
        unimplemented!()
    }
    fn tag_timeline(&self, hashtag: String, query: String) -> Result<Vec<entities::Status>, &str> {
        unimplemented!()
    }
}
