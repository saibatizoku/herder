extern crate herder;
extern crate url;

use herder::Client;
use herder::Mastodon;
use herder::mastodon::{NodeInstance, ApiHandler};
use url::Url;

const BASE_URL: &str = "http://localhost:3000/";
const MY_TOKEN: &str = "MY_TOKEN";

#[test]
fn mastodon_node_created() {
    let Mastodon(domain) = Mastodon::new(BASE_URL).unwrap();
    assert_eq!(domain, Url::parse(BASE_URL).unwrap());
}

#[test]
fn create_client_with_mastodon_domain() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let Client { url_base, .. } = client;
    assert_eq!(url_base, Url::parse(BASE_URL).unwrap());
}

#[test]
fn create_client_with_bearer_token() {
    let Client { token, .. } = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    assert_eq!(token, String::from(MY_TOKEN));
}

// Testing of ApiHandler trait on Client
#[test]
fn client_builds_api_endpoint_urls() {
    let client = Mastodon::new(BASE_URL).unwrap().client(MY_TOKEN).unwrap();
    let api_endpoint = client.endpoint_url_string("/api/v1/");
    assert_eq!(api_endpoint, format!("{}/api/v1/", BASE_URL));
}

#[test]
fn client_builds_api_request() {
    unimplemented!()
}
