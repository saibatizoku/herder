//! This module contains the code representing Mastodon nodes and API Clients
//!
use Client;
use api::oauth::{CreateApp, OAuthApp};
use errors::*;
use hyper::header::Bearer;
use serde_json;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use url::Url;

/// `Mastodon` is used to specify the base url of a Mastodon node.
/// Only HTTPS connections are supported.
#[derive(Debug)]
pub struct Mastodon(pub Url);

/// Methods to interact with a Mastodon instance.
pub trait NodeInstance {
    /// Create a new Mastodon instance.
    fn new(url: &str) -> Result<Mastodon>;
    /// Returns the base `Url` of the Mastodon instance.
    fn url(&self) -> Result<Url>;
    /// Returns a Client for the API of the Mastodon instance.
    fn client(&self, token: &str) -> Result<Client>;
    /// Register a 3rd-party App with the Mastodon instance.
    fn register_app(&self, app: CreateApp) -> Result<OAuthApp>;
}

impl NodeInstance for Mastodon {
    fn new(url: &str) -> Result<Mastodon> {
        Ok(Mastodon(Url::parse(url).chain_err(|| "Invalid URL")?))
    }
    fn url(&self) -> Result<Url> {
        Ok(self.0.clone())
    }
    fn client(&self, token: &str) -> Result<Client> {
        Ok(Client {
            url_base: self.url().chain_err(|| "Could not set the base URL")?,
            token: Bearer::from_str(token).chain_err(|| "Could parse Bearer Token")?
        })
    }
    fn register_app(&self, app: CreateApp) -> Result<OAuthApp> {
        let out = Arc::new(Mutex::new(Vec::new()));
        app
            .register(&self.endpoint_url_string("/api/v1/apps"), out.clone())
            .chain_err(|| "Could not register App.")?;
        let out = out.lock().unwrap();
        if out.is_empty() { bail!("Invalid result. Empty") }
        serde_json::from_slice(&out).chain_err(|| "Unexpected JSON error.")
    }
}

pub trait ApiHandler {
    fn endpoint_url(&self, path: &str) -> Result<Url>;
    fn endpoint_url_string(&self, path: &str) -> String;
}

impl ApiHandler for Mastodon {
    fn endpoint_url(&self, path: &str) -> Result<Url> {
        self.0.clone().join(path).chain_err(|| "invalid endpoint URL")
    }
    fn endpoint_url_string(&self, path: &str) -> String {
        self.endpoint_url(path).chain_err(|| "invalid endpoint URL").unwrap().into_string()
    }
}
