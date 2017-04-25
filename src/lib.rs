//! herder v0.1.0
//!
extern crate curl;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

#[macro_use] extern crate serde_derive;

pub mod api;
pub mod client;
pub mod oauth;

pub use client::Mastodon;
pub use oauth::OAuthApp;
