//! This module contains the supported versions of the Mastodon API.
//! Currently only `/api/v1`.
use hyper;

pub mod v1;
pub mod oauth;

pub use self::v1::client::Client;

/// constructor of HTTPS requests, used in API Mehtods.
///
#[derive(Debug, Default, PartialEq)]
pub struct APIMethod {
    /// defines the HTTP Request method
    pub request_method: hyper::Method,
    /// API endpoint for this method
    pub endpoint: String,
    /// API endpoint for this method
    pub form_data: Option<String>,
    /// API endpoint for this method
    pub url_query: Option<String>
}

