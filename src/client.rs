use oauth::OAuthApp;

#[derive(Debug)]
pub struct Mastodon {
    pub endpoint: String,
    pub oauthapp: Option<OAuthApp>,
}

impl Mastodon {
    pub fn new(url: &str) -> Mastodon {
        Mastodon {
            endpoint: String::from(url),
            oauthapp: None
        }
    }

}
