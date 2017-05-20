//! This module contains the code representing Mastodon nodes and API Clients
//!
use errors::*;
use futures::{Future, Stream};
use hyper::Client as WebClient;
use hyper::{Body, Uri, Method};
use hyper::Method::{Get, Patch, Post};
use hyper::client::Request;
use hyper::header::{Headers, ContentType, Authorization, Bearer};
use hyper_tls::HttpsConnector;
use mastodon::ApiHandler;
use serde_json;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use super::entities;
use super::methods;
use super::methods::{
    AccountID,
    APIEndpoint,
    APIEndpointRequest,
    HomeTimelineQuery,
    NotificationID,
    RelationshipsQuery,
    SearchAccountsQuery,
    SearchContentsQuery,
    StatusID,
    TagID,
    TimelineQuery,
    UserFormData
};
use tokio_core::reactor::Core;
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
    fn bearer_token_request(&self, method: Method, uri: Uri) -> Result<Request<Body>> {
        let mut req = Request::new(method, uri);
        req.headers_mut().set(Authorization(self.token.clone()));
        Ok(req)
    }
    fn build_request(&self, endpoint: APIEndpoint) -> Result<Request<Body>> {
        match endpoint {
            APIEndpoint::FetchAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::GetCurrentUser => {
                let url = self.endpoint_url("/api/v1/accounts/verify_credentials")?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::UpdateCurrentUser(d) => {
                let url = self.endpoint_url("/api/v1/accounts/update_credentials")?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Patch, uri)
            },
            APIEndpoint::GetAccountFollowers(a) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/followers", a.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::GetFollowing(a) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/following", a.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::GetAccountStatuses(a) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/statuses", a.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::FollowAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/follow", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Post, uri)
            },
            APIEndpoint::UnfollowAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/unfollow", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Post, uri)
            },
            APIEndpoint::BlockAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/block", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::UnblockAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/unblock", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                Ok(Request::new(Post, uri))
            },
            APIEndpoint::MuteAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/mute", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Post, uri)
            },
            APIEndpoint::UnmuteAccount(account) => {
                let url = self.endpoint_url(&format!("/api/v1/accounts/{}/unmute", account.id))?;
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Post, uri)
            },
            APIEndpoint::GetAccountRelationships(query) => {
                let mut url = self.endpoint_url("/api/v1/accounts/relationships").unwrap();
                match query {
                    RelationshipsQuery::SingleAccount(account) => {
                        url.query_pairs_mut().append_pair("id", &format!("{}", account.id));
                    },
                    RelationshipsQuery::MultipleAccounts(accounts) => {
                        for account in &accounts {
                            url.query_pairs_mut()
                                .append_pair("id[]", &format!("{}", account.id));
                        }
                    }
                };
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            },
            APIEndpoint::SearchAccounts(q) => {
                let mut url = self.endpoint_url("/api/v1/accounts/search").unwrap();
                url.query_pairs_mut().append_pair("q", &q.q);
                if let Some(limit) = q.limit {
                    url.query_pairs_mut().append_pair("limit", &format!("{}", limit));
                };
                let uri = Uri::from_str(url.as_str()).unwrap();
                self.bearer_token_request(Get, uri)
            }
        }
    }

    fn send(&self, endpoint: APIEndpoint, dst: Arc<Mutex<Vec<u8>>>) -> Result<()> {
        let mut core = Core::new().chain_err(|| "Could not start client reactor")?;
        let client = WebClient::configure()
            .connector(HttpsConnector::new(4, &core.handle()))
            .build(&core.handle());

        let req: Request<Body> = self.build_request(endpoint).chain_err(|| "Could not build request")?;
        //req.headers_mut().set(ContentType::form_url_encoded());
        //req.set_body(Body::from(self.form_encode()));

        let mut dst = dst.lock().unwrap();
        let work = client.request(req)
            .and_then(|res| {
                res.body().for_each(|chunk| {
                    dst.extend_from_slice(&chunk);
                    Ok(())
                })
            });
        core.run(work).chain_err(|| "Failed to run registration")?;
        Ok(())
    }
}

impl methods::Accounts for Client {
    fn fetch_account(&self, account_id: AccountID) -> Result<entities::Account> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::FetchAccount(account_id);
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error fetching account.")
    }
    fn get_current_user(&self) -> Result<entities::Account> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::GetCurrentUser;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting current user.")
    }
    fn update_current_user(&self, form_data: UserFormData) -> Result<entities::Account> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::UpdateCurrentUser(form_data);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error updating current user.")
    }
    fn get_account_followers(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::GetAccountFollowers(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting account followers.")
    }
    fn get_account_following(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::GetFollowing(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting account's following.")
    }
    fn get_account_statutes(&self, account_id: AccountID) -> Result<Vec<entities::Status>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::GetAccountStatuses(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting account's statuses.")
    }
    fn follow_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::FollowAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error following account.")
    }
    fn unfollow_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::UnfollowAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error unfollowing account.")
    }
    fn block_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::BlockAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error blocking account.")
    }
    fn unblock_account(&self, account_id: AccountID) -> Result<Vec<entities::Account>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::UnblockAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error unblocking account.")
    }
    fn mute_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::MuteAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error muting account.")
    }
    fn unmute_account(&self, account_id: AccountID) -> Result<entities::Relationship> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::UnmuteAccount(account_id);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error unmuting account.")
    }
    fn get_account_relationships(&self, query: RelationshipsQuery) -> Result<Vec<entities::Relationship>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::GetAccountRelationships(query);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting account relationships.")
    }
    fn search_accounts(&self, query: SearchAccountsQuery) -> Result<Vec<entities::Account>> {
        let data = Arc::new(Mutex::new(Vec::new()));
        let endpoint = APIEndpoint::SearchAccounts(query);;
        self.send(endpoint, data.clone()).unwrap();
        let data = data.lock().unwrap();
        serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error getting account search.")
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
