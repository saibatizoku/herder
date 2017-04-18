extern crate herder;

use herder::Mastodon;
use herder::client::{Client, ApiHandler};

const BASE_URL: &'static str = "http://localhost:3000/";

#[test]
fn mastodon_node_created() {
    let Mastodon(domain) = Mastodon::new(BASE_URL);
    assert_eq!(domain, String::from(BASE_URL));
}

#[test]
fn create_client_with_mastodon_domain() {
    let client = Mastodon::new(BASE_URL).client("MYTOKEN");
    let Client { url_base, .. } = client;
    assert_eq!(url_base, String::from(BASE_URL));
}

#[test]
fn create_client_with_bearer_token() {
    let client = Mastodon::new(BASE_URL).client("MYTOKEN");
    let Client { token, .. } = client;
    assert_eq!(token, String::from("MYTOKEN"));
}

// Testing of ApiHandler trait on Client
#[test]
fn client_builds_api_endpoint_urls() {
    let client = Mastodon::new(BASE_URL).client("MYTOKEN");
    let api_endpoint = client.endpoint_url_string("/api/v1/");
    assert_eq!(api_endpoint, String::from("http://localhost:3000/api/v1/"));
}

#[test]
fn client_publish_a_toot() {
    unimplemented!()
}
