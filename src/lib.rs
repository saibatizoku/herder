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
#[macro_use] extern crate error_chain;

pub mod api;
pub mod client;
pub mod errors {
    //! Setup `error-chain`
    error_chain!{}
}

pub use client::{Client, Mastodon};
pub use errors::*;
