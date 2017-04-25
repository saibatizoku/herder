//! `herder` is a crate that provides asynchronous clients for Mastodon, a GNU-Social
//! compatible microblogging service.
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

/// Use `Client` to interact with `Mastodon`.
pub use client::Client;

/// Use `Mastodon` to define the server you wish to connect to. Only HTTPS is supported.
pub use client::Mastodon;

/// Use `OAuthApp` to register your `Client` on `Mastodon`.
pub use oauth::OAuthApp;
