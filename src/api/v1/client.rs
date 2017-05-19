//! This module contains the code representing Mastodon nodes and API Clients
//!
use api::APIMethodRequest;
use errors::*;
use hyper::Uri;
use hyper::header::Bearer;
use hyper::Client as WebClient;
use hyper::{Body, Uri};
use hyper::Method::{Get, Patch, Post};
use hyper::client::Request;
use mastodon::ApiHandler;
use std::str::FromStr;
use super::entities;
use super::methods;
use super::methods::{
    AccountID,
    APIEndpoint,
    APIEndpointRequest,
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

impl APIEndpointRequest for Client {
    fn build_request(&self, endpoint: APIEndpoint) -> Result<Request<Body>> {
        match endpoint {
            APIEndpoint::FetchAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}", account.id));
                let uri = Uri::from_str(&url).unwrap();
                let mut req = Request::new(Get, uri);
                Ok(req)
            },
            APIEndpoint::GetCurrentUser => {
                let url = self.endpoint_url_string("/api/v1/accounts/verify_credentials");
                let uri = Uri::from_str(&url).unwrap();
                let mut req = Request::new(Get, uri);
                Ok(req)
            },
            APIEndpoint::UpdateCurrentUser(d) => {
                let url = self.endpoint_url_string("/api/v1/accounts/update_credentials");
                let uri = Uri::from_str(&url).unwrap();
                let mut req = Request::new(Patch, uri);
                Ok(req)
            },
            APIEndpoint::GetAccountFollowers(a) => {
                let uri = Uri::from_str(&format!("/api/v1/accounts/{}/followers", a.id)).unwrap();
                let mut req = Request::new(Get, uri);
                Ok(req)
            },
            APIEndpoint::GetFollowing(a) => {
                let uri = Uri::from_str(&format!("/api/v1/accounts/{}/following", a.id)).unwrap();
                Ok(Request::new(Get, uri))
            },
            APIEndpoint::GetAccountStatuses(a) => {
                let uri = Uri::from_str(&format!("/api/v1/accounts/{}/statuses", a.id)).unwrap();
                Ok(Request::new(Get, uri))
            },
            APIEndpoint::FollowAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/follow", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::UnfollowAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/unfollow", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::BlockAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/block", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::UnblockAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/unblock", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::MuteAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/mute", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::UnmuteAccount(account) => {
                let url = self.endpoint_url_string(&format!("/api/v1/accounts/{}/unmute", account.id));
                let uri = Uri::from_str(&url).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::GetAccountRelationships(query) => {
                let mut url = self.endpoint_url("/api/v1/accounts/relationships").unwrap();
                match query {
                    RelationshipsQuery::SingleAccount(account) => {
                        url.query_pairs_mut().append_pair("id", &format!("{}", account.id));
                    },
                    RelationshipsQuery::MultipleAccounts(accounts) => {
                        for account in &accounts {
                            url.query_pairs_mut().append_pair("id[]", &format!("{}", account.id));
                        }
                    }
                };
                let req = Request::new(Get, Uri::from_str(url.as_str()).unwrap());
                Ok(req)
            },
            APIEndpoint::SearchAccounts(q) => {
                let mut url = self.endpoint_url("/api/v1/accounts/search").unwrap();
                url.query_pairs_mut().append_pair("q", &q.q);
                if let Some(limit) = q.limit {
                    url.query_pairs_mut().append_pair("limit", &format!("{}", limit));
                };
                let uri = Uri::from_str(url.as_str()).unwrap();
                Ok(Request::new(Get, uri))
            }
        }
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
