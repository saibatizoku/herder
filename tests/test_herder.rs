extern crate herder;

use herder::Mastodon;

#[test]
fn mastodon_node_created() {
    let mastodon_node = Mastodon::new("http://localhost");
    assert_eq!(mastodon_node.endpoint, String::from("http://localhost"));
}
