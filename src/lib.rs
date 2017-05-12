//! a library that provides asynchronous clients for Mastodon, a GNU-Social
//! compatible microblogging service.
//!
extern crate curl;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

pub mod api;
pub mod mastodon;
pub mod errors;

pub use api::Client;
pub use mastodon::Mastodon;
