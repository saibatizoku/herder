//! This module contains the code representing Mastodon nodes and API Clients
//!
use serde_json;
use url::Url;

use api::oauth::{CreateApp, OAuthApp};
use api::v1::client::Client;

use std::sync::{Arc, Mutex};

/// `Mastodon` is used to specify the base url of a Mastodon node.
/// Only HTTPS connections are supported.
#[derive(Debug)]
pub struct Mastodon(pub String);

impl Mastodon {
    pub fn new(url: &str) -> Mastodon {
        Mastodon(String::from(url))
    }

    pub fn url(&self) -> String {
        Url::parse(&self.0).unwrap().into_string()
    }

    pub fn client(&self, token: &str) -> Client {
        Client {
            url_base: String::from(self.url()),
            token: String::from(token)
        }
    }

    pub fn register_app(&self, name: &str, uris: &str, scopes: &str) -> OAuthApp {
        /// ```rust,no_run
        /// let herder_app: OAuthApp = serde_json::from_slice(&result).unwrap();
        /// println!("\n{:?}\n", &herder_app);
        /// println!("\n{}\n", serde_json::to_string(&herder_app).unwrap());
        /// ```
        let out = Arc::new(Mutex::new(Vec::new()));
        let social_app = CreateApp::new(name, uris, scopes);
        social_app.register(&self.endpoint_url_string("/api/v1/apps"), out.clone()).unwrap();

        let result = out.lock().unwrap();
        let json: OAuthApp = serde_json::from_slice(&result).expect("Could not parse OAuth from response");
        //format!("{}", &json)
        json
    }
}

pub trait ApiHandler {
    fn endpoint_url_string(&self, path: &str) -> String;
}

impl ApiHandler for Mastodon {
    fn endpoint_url_string(&self, path: &str) -> String {
        let base = Url::parse(&self.url()).unwrap();
        base.join(path).unwrap().into_string()
    }
}
