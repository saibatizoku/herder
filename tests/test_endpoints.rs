extern crate herder;
extern crate hyper;
extern crate url;

use herder::Mastodon;
use herder::api::v1::methods::{
    AccountID,
    APIEndpoint,
    APIEndpointRequest,
    RelationshipsQuery,
    SearchAccountsQuery,
    UserFormData
};
use herder::mastodon::NodeInstance;
use hyper::Method::{Get, Patch, Post};

const BASE_URL: &str = "http://localhost:3000";
const MY_TOKEN: &str = "MY_TOKEN";

// APIEndpoint::FetchAccount(AccountID)
#[test]
fn api_request_fetch_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::FetchAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345")
}

// APIEndpoint::GetCurrentUser,
#[test]
fn api_request_get_current_user() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetCurrentUser;
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/verify_credentials")
}

// APIEndpoint::UpdateCurrentUser(UserFormData)
#[test]
fn api_request_update_current_user() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UpdateCurrentUser(UserFormData::new(None, None, None, None));
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Patch);
    assert_eq!(request.uri().path(), "/api/v1/accounts/update_credentials")
}

// APIEndpoint::GetAccountFollowers(AccountID)
#[test]
fn api_request_get_account_followers() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetAccountFollowers(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/followers")
}

// APIEndpoint::GetFollowing(AccountID)
#[test]
fn api_request_get_account_following() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetFollowing(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/following")
}

// APIEndpoint::GetAccountStatuses(AccountID)
#[test]
fn api_request_get_account_statuses() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::GetAccountStatuses(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/statuses")
}

// APIEndpoint::FollowAccount(AccountID)
#[test]
fn api_request_following_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::FollowAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/follow")
}

// APIEndpoint::UnfollowAccount(AccountID)
#[test]
fn api_request_unfollowing_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnfollowAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unfollow")
}

// APIEndpoint::BlockAccount(AccountID)
#[test]
fn api_request_block_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::BlockAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/block")
}

// APIEndpoint::UnblockAccount(AccountID)
#[test]
fn api_request_unblock_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnblockAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unblock")
}

// APIEndpoint::MuteAccount(AccountID)
#[test]
fn api_request_mute_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::MuteAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/mute")
}

// APIEndpoint::UnmuteAccount(AccountID)
#[test]
fn api_request_unmute_an_account() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::UnmuteAccount(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/unmute")
}

// APIEndpoint::GetAccountRelationships(RelationshipsQuery::Single)
#[test]
fn api_request_single_account_relationship() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let query = RelationshipsQuery::Single(AccountID { id: 12345 });
    let endpoint = APIEndpoint::GetAccountRelationships(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/relationships")
}

// APIEndpoint::GetAccountRelationships(RelationshipsQuery::Many)
#[test]
fn api_request_many_accounts_relationship() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let accounts = vec![AccountID { id: 12345 }, AccountID { id: 67890 }];
    let query = RelationshipsQuery::Many(accounts);
    let endpoint = APIEndpoint::GetAccountRelationships(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/relationships")
}

// APIEndpoint::SearchAccounts(SearchAccountsQuery)
#[test]
fn api_request_search_accounts() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let query = SearchAccountsQuery { q: String::from("search text"), limit: None };
    let endpoint = APIEndpoint::SearchAccounts(query);
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Get);
    assert_eq!(request.uri().path(), "/api/v1/accounts/search")
}
#[test]
// APIEndpoint::FetchBlocks(AccountID)
fn api_request_fetch_blocks() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let endpoint = APIEndpoint::FetchBlocks(AccountID { id: 12345 });
    let request = client.build_request(endpoint).unwrap();
    assert_eq!(*request.method(), Post);
    assert_eq!(request.uri().path(), "/api/v1/accounts/12345/mute")
}
// APIEndpoint::FetchFollowRequests(AccountID)
#[test]
fn api_request_fetch_favourites() {
    unimplemented!()
}
// APIEndpoint::FetchFollowRequests(Query)
#[test]
fn api_request_fetch_follow_requests() {
    unimplemented!()
}
// APIEndpoint::AuthorizeFollowRequest(FormData)
#[test]
fn api_request_authorize_follow_request(){
}
// APIEndpoint::RejectFollowRequest(FormData)
#[test]
fn api_request_reject_follow_request() {
    unimplemented!()
}
// APIEndpoint::FollowRemoteUser(FormData)
#[test]
fn api_request_follow_remote_user() {
    unimplemented!()
}
// APIEndpoint::GetInstance
#[test]
fn api_request_get_instance() {
    unimplemented!()
}
// APIEndpoint::UploadMedia
#[test]
fn api_request_upload_media() {
    unimplemented!()
}
// APIEndpoint::FetchMutes(Query)
#[test]
fn api_request_fetch_mutes() {
    unimplemented!()
}
// APIEndpoint::FetchNotifications(Query)
#[test]
fn api_request_fetch_notifications() {
    unimplemented!()
}
// APIEndpoint::GetNotification(NotificationID)
#[test]
fn api_request_get_notification() {
    unimplemented!()
}
// APIEndpoint::ClearNotifications
#[test]
fn api_request_clear_notifications() {
    unimplemented!()
}
// APIEndpoint::FetchingReports
#[test]
fn api_request_fetching_reports() {
    unimplemented!()
}
// APIEndpoint::ReportingUser(UserReport)
#[test]
fn api_request_reporting_user() {
    unimplemented!()
}
// APIEndpoint::SearchContent(ContentQuery)
#[test]
fn api_request_search_content() {
    unimplemented!()
}
// APIEndpoint::FetchStatus(StatusID)
#[test]
fn api_request_fetch_status() {
    unimplemented!()
}
// APIEndpoint::GetStatusContext(StatusID)
#[test]
fn api_request_get_status_context() {
    unimplemented!()
}
// APIEndpoint::GetStatusCard(StatusID)
#[test]
fn api_request_get_status_card() {
    unimplemented!()
}
// APIEndpoint::RebloggedBy(StatusID)
#[test]
fn api_request_reblogged_by() {
    unimplemented!()
}
// APIEndpoint::FavouritedBy(StatusID)
#[test]
fn api_request_favourited_by() {
    unimplemented!()
}
// APIEndpoint::PostStatus(PostFormData)
#[test]
fn api_request_post_status() {
    unimplemented!()
}
// APIEndpoint::DeleteStatus(StatusID)
#[test]
fn api_request_delete_status() {
    unimplemented!()
}
// APIEndpoint::ReblogStatus(StatusID)
#[test]
fn api_request_reblog_status() {
    unimplemented!()
}
// APIEndpoint::UnreblogStatus(StatusID)
#[test]
fn api_request_unreblog_status() {
    unimplemented!()
}
// APIEndpoint::FavouriteStatus(StatusID)
#[test]
fn api_request_favourite_status() {
    unimplemented!()
}
// APIEndpoint::UnfavouriteStatus(StatusID)
#[test]
fn api_request_unfavourite_status() {
    unimplemented!()
}
// APIEndpoint::PublicTimeLine(PublicQuery)
#[test]
fn api_request_home_timeline() {
    unimplemented!()
}
// APIEndpoint::PublicTimeLine(PublicQuery)
#[test]
fn api_request_public_timeline() {
    unimplemented!()
}
// APIEndpoint::TagTimeLine(Tag, TagQuery)
#[test]
fn api_request_tag_timeline() {
    unimplemented!()
}
