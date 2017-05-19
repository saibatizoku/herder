extern crate herder;
extern crate hyper;
extern crate url;

use herder::{Client, Mastodon};
use herder::api::v1::methods::{
    AccountID,
    APIEndpoint,
    APIEndpointRequest,
    RelationshipsQuery,
    SearchAccountsQuery,
    UserFormData
};
use herder::mastodon::{NodeInstance, ApiHandler};
use hyper::header::Bearer;
use hyper::Method::{Get, Patch, Post};
use std::str::FromStr;
use url::Url;

const BASE_URL: &str = "http://localhost:3000";
const MY_TOKEN: &str = "MY_TOKEN";

#[test]
fn mastodon_node_created() {
    let Mastodon(domain) = Mastodon::new(BASE_URL).unwrap();
    assert_eq!(domain, Url::parse(BASE_URL).unwrap());
}

#[test]
fn create_client_with_mastodon_domain() {
    let mastodon = Mastodon::new(BASE_URL).unwrap();
    let Client { url_base, .. } = mastodon.client(MY_TOKEN).unwrap();
    assert_eq!(url_base, Url::parse(BASE_URL).unwrap());
}

#[test]
fn create_client_with_bearer_token() {
    let mastodon = Mastodon::new(BASE_URL).unwrap();
    let Client { token, .. } = mastodon.client(MY_TOKEN).unwrap();
    assert_eq!(token, Bearer::from_str(MY_TOKEN).unwrap());
}

// Testing of ApiHandler trait on Client
#[test]
fn client_builds_api_endpoint_url() {
    let mastodon = Mastodon::new(BASE_URL).unwrap();
    let client = mastodon.client(MY_TOKEN).unwrap();
    let api_endpoint = client.endpoint_url("/api/v1/").unwrap();
    assert_eq!(api_endpoint, Url::parse(&format!("{}/api/v1/", BASE_URL)).unwrap());
}

// Testing API Method Requests
// pub enum APIEndpoint {
//     // AccountsMethod: Fetch Account
//     FetchAccount(AccountID),
//     GetCurrentUser,
//     UpdateCurrentUser(UserFormData),
//     GetAccountFollowers(AccountID),
//     GetFollowing(AccountID),
//     GetAccountStatuses(AccountID),
//     FollowAccount(AccountID),
//     UnfollowAccount(AccountID),
//     BlockAccount(AccountID),
//     UnblockAccount(AccountID),
//     MuteAccount(AccountID),
//     UnmuteAccount(AccountID),
//     GetAccountRelationships(RelationshipsQuery),
//     SearchAccounts(SearchAccountsQuery)
// }

#[test]
fn api_request_fetch_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::FetchAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345")
}

#[test]
fn api_request_get_current_user() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetCurrentUser;
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/verify_credentials")
}

#[test]
fn api_request_update_current_user() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UpdateCurrentUser(UserFormData::new(None, None, None, None));
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Patch);
    assert_eq!(request.uri().path(), "/api/v1/accounts/update_credentials")
}

#[test]
fn api_request_get_account_followers() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetAccountFollowers(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/followers")
}

#[test]
fn api_request_get_account_following() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetFollowing(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/following")
}

#[test]
fn api_request_get_account_statuses() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetAccountStatuses(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/statuses")
}

#[test]
fn api_request_following_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::FollowAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/follow")
}

#[test]
fn api_request_unfollowing_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnfollowAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unfollow")
}

#[test]
fn api_request_block_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::BlockAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/block")
}

#[test]
fn api_request_unblock_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnblockAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unblock")
}

#[test]
fn api_request_mute_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::MuteAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/mute")
}

#[test]
fn api_request_unmute_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnmuteAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unmute")
}

#[test]
fn api_request_single_account_relationship() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let query = RelationshipsQuery::SingleAccount(AccountID { id: 12345 });
    let endpoint = APIEndpoint::GetAccountRelationships(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/relationships")
}

#[test]
fn api_request_many_accounts_relationship() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let accounts = vec![AccountID { id: 12345 }, AccountID { id: 67890 }];
    let query = RelationshipsQuery::MultipleAccounts(accounts);
    let endpoint = APIEndpoint::GetAccountRelationships(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/relationships")
}

#[test]
fn api_request_search_accounts() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let query = SearchAccountsQuery { q: String::from("search text"), limit: None };
    let endpoint = APIEndpoint::SearchAccounts(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/search")
}
