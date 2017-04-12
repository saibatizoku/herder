extern crate herder;

use herder::entities::Mastodon;
use herder::oauth::OAuthApp;

#[test]
fn mastodon_node_created() {
    let mastodon_node = Mastodon::new("http://localhost");
    assert_eq!(mastodon_node.endpoint, String::from("http://localhost"));
}

#[test]
fn client_registration() {
    unimplemented!()
}

#[test]
fn client_token_authentication() {
    unimplemented!()
}
