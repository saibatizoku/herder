//! herder v0.1.0
//!
extern crate curl;
extern crate serde_json;
extern crate url;

#[macro_use] extern crate serde_derive;

pub mod api;
pub mod client;
pub mod oauth;

pub use client::Mastodon;
pub use oauth::OAuthApp;
