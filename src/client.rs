//! This module contains the code representing Mastodon nodes and API Clients
//!
use super::oauth::{CreateApp, OAuthApp};
use url::Url;
use serde_json;

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
    pub fn create_app(&self, name: &str, uris: &str, scopes: &str) -> OAuthApp {
        /// ```rust,no_run
        /// let herder_app: OAuthApp = serde_json::from_slice(&result).unwrap();
        /// println!("\n{:?}\n", &herder_app);
        /// println!("\n{}\n", serde_json::to_string(&herder_app).unwrap());
        /// ```
        let out = Arc::new(Mutex::new(Vec::new()));
        let social_app = CreateApp::new(name, uris, scopes);
        social_app.register_app(&self.endpoint_url_string("/api/v1/apps"), out.clone()).unwrap();

        let result = out.lock().unwrap();
        serde_json::from_slice(&result).expect("Could not parse OAuth from response")
    }
}

/// The API Client, currently works for version 1 of the Mastodon API.
pub struct Client {
    pub url_base: String,
    pub token: String
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

impl ApiHandler for Client {
    fn endpoint_url_string(&self, path: &str) -> String {
        let base = Url::parse(&self.url_base).unwrap();
        base.join(path).unwrap().into_string()
    }
}
