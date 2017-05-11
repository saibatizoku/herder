//! This module contains the code representing Mastodon nodes and API Clients
//!
use serde_json;
use url::Url;

use super::api::APIMethod;
use super::api::oauth::{CreateApp, OAuthApp};
use super::api::v1::entities;
use super::api::v1::methods;

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

impl methods::Accounts for Client {
    fn fetch_account(&self, account_id: usize) -> Result<entities::Account, &str> {
        let api_method = APIMethod {
            endpoint: format!("/api/v1/accounts/{}", account_id),
            ..APIMethod::default()
        };
        println!("FETCH ACCOUNT: {:?}", api_method);
        Ok(entities::Account::default())
    }
    fn get_current_user(&self) -> Result<entities::Account, &str> {
        unimplemented!();
    }
    fn update_current_user(&self, form_data: String) -> Result<entities::Account, &str> {
        unimplemented!()
    }
    fn get_account_followers(&self, account_id: usize) -> Result<Vec<entities::Account>, &str> {
        unimplemented!()
    }
}
