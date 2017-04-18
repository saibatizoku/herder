extern crate herder;

use herder::Mastodon;
use herder::client::Client;

#[test]
fn mastodon_node_created() {
    let Mastodon(mastodon_url) = Mastodon::new("http://localhost");
    assert_eq!(mastodon_url, String::from("http://localhost"));
}

#[test]
fn create_client_with_mastodon_domain() {
    let mastodon = Mastodon::new("http://localhost");
    let Client { url_base: mastodon_domain, .. } = mastodon.client(&mastodon.0, "MYTOKEN");
    assert_eq!(mastodon_domain, String::from("http://localhost"));
}

#[test]
fn create_client_with_bearer_token() {
    let mastodon = Mastodon::new("http://localhost");
    let Client { bearer_token: token, .. } = mastodon.client(&mastodon.0, "MYTOKEN");
    assert_eq!(token, String::from("MYTOKEN"));
}
