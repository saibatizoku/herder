//! This module contains the supported versions of the Mastodon API.
//! Currently only `/api/v1`.
use hyper;

pub mod v1;
pub mod oauth;

pub use self::v1::client::Client;

/// constructor of HTTPS requests, used in API Mehtods.
///
#[derive(Debug, Default, PartialEq)]
pub struct APIMethodRequest {
    /// defines the HTTP Request method
    pub body: Option<String>,
    /// query of request
    pub headers: hyper::Headers,
    /// body of request
    pub method: hyper::Method,
    /// body of request
    pub query: Option<String>,
    /// API endpoint for this method
    pub uri: hyper::Uri
}
