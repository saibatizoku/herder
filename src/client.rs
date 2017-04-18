extern crate tokio_core;
extern crate curl;
extern crate futures;
extern crate tokio_curl;

// use self::tokio_core::reactor::Core;
// use self::tokio_curl::Session;

use oauth::OAuthApp;

#[derive(Debug)]
pub struct Mastodon(pub String);

pub struct Client {
    pub url_base: String,
    pub bearer_token: String
}

impl Mastodon {
    pub fn new(url: &str) -> Mastodon {
        Mastodon(String::from(url))
    }
    pub fn client(&self, url_base: &str, token: &str) -> Client {
        Client {
            url_base: String::from(url_base),
            bearer_token: String::from(token)
        }
    }
}
