extern crate herder;

#[test]
fn create_client() {
    let mastodon_node = herder::Mastodon::new("http://localhost");
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
