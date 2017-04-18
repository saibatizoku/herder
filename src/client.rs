extern crate url;

use self::url::Url;

#[derive(Debug)]
pub struct Mastodon(pub String);

impl Mastodon {
    pub fn new(url: &str) -> Mastodon {
        Mastodon(String::from(url))
    }

    pub fn url(&self) -> &str {
        &self.0
    }

    pub fn client(&self, token: &str) -> Client {
        Client {
            url_base: String::from(self.url()),
            bearer_token: String::from(token)
        }
    }

    pub fn endpoint_url(&self, path: &str) -> String {
        let base = Url::parse(&self.0).unwrap();
        let url = base.join(path).unwrap();
        url.into_string()
    }
}

pub struct Client {
    pub url_base: String,
    pub bearer_token: String
}
