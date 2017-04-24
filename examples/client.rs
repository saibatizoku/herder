extern crate futures;
extern crate herder;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use herder::Mastodon;
use herder::oauth::OAuthApp;
use hyper::{Client, Body, Post};
use hyper::client::Request;
use hyper::header::ContentType;
use futures::{Future, Stream};
use url::form_urlencoded;

use std::io::Write;
use std::sync::{Arc, Mutex};

fn main() {
    let new_app = CreateApp {
        client_name: String::from("herder"),
        redirect_uris: String::from("urn:ietf:wg:oauth:2.0:oob"),
        scopes: String::from("read write follow")
    };

    let herder_app = CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");

    let social_app = CreateApp::default();

    println!("new app: {:?}", new_app);
    println!("new herder app: {:?}", herder_app);
    println!("new herder social app: {:?}", social_app);

    // let myapp = get_new_client();
    // println!("out:\n{}",
    //          serde_json::to_string(&myapp).unwrap());
}

#[derive(Debug)]
struct CreateApp {
    client_name: String,
    redirect_uris: String,
    scopes: String
}

impl Default for CreateApp {
    fn default() -> Self {
        CreateApp {
            client_name: String::from("herder"),
            redirect_uris: String::from("urn:ietf:wg:oauth:2.0:oob"),
            scopes: String::from("read")
        }
    }
}

impl CreateApp {
    fn new(name: &str, uris: &str, scopes: &str) -> CreateApp {
        CreateApp {
            client_name: String::from(name),
            redirect_uris: String::from(uris),
            scopes: String::from(scopes)
        }
    }

    fn form_encode(&self) -> &str {
        ""
    }
}
