extern crate herder;

use herder::Mastodon;
use herder::client::Client;

#[test]
fn mastodon_node_created() {
    let Mastodon(domain) = Mastodon::new("http://localhost:3000/");
    assert_eq!(domain, String::from("http://localhost:3000/"));
}

#[test]
fn create_client_with_mastodon_domain() {
    let mastodon = Mastodon::new("http://localhost:3000/");
    let Client { url_base: domain, .. } = mastodon.client("MYTOKEN");
    assert_eq!(domain, String::from("http://localhost:3000/"));
}

#[test]
fn create_client_with_bearer_token() {
    let mastodon = Mastodon::new("http://localhost:3000/");
    let Client { bearer_token: token, .. } = mastodon.client("MYTOKEN");
    assert_eq!(token, String::from("MYTOKEN"));
}

#[test]
fn client_builds_api_endpoints() {
    let mastodon = Mastodon::new("http://localhost:3000/");
    let api_endpoint = mastodon.endpoint_url("/api/v1/");
    assert_eq!(api_endpoint, String::from("http://localhost:3000/api/v1/"));
}

#[test]
fn client_publish_a_toot() {
    unimplemented!()
}
