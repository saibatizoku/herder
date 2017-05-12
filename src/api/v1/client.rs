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
    fn block_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn unblock_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
impl methods::Favourites for Client {
    fn favourite_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
    fn unfavourite_account(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
impl methods::FollowRequests for Client {
}

impl methods::Follows for Client {
}

impl methods::Instances for Client {
}

impl methods::Media for Client {
}

impl methods::Mutes for Client {
}

impl methods::Notifications for Client {
}

impl methods::Reports for Client {
}

impl methods::Search for Client {
}

impl methods::Statuses for Client {
}

impl methods::Timelines for Client {
}
