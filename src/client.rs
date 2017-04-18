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
            token: String::from(token)
        }
    }
}

pub struct Client {
    pub url_base: String,
    pub token: String
}

pub trait ApiHandler {
    fn endpoint_url_string(&self, path: &str) -> String;
}

impl ApiHandler for Client {
    fn endpoint_url_string(&self, path: &str) -> String {
        let base = Url::parse(&self.url_base).unwrap();
        base.join(path).unwrap().into_string()
    }
}
