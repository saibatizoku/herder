extern crate herder;
extern crate hyper;
extern crate url;

use herder::{Client, Mastodon};
use herder::mastodon::{NodeInstance, ApiHandler};
use hyper::header::Bearer;
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
