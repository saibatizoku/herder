//! A simple program to register a client app on a Mastodon endpoint.
//!
//! This example uses a simple curl-based client to register a herder client
//! on a given Mastodon endpoint., post form data to it, and
//! stdout and saves the resulting JSON onto a given filepath.

extern crate herder;
extern crate serde_json;

use herder::api::oauth::OAuthApp;

use std::fs::File;

fn main() {
    let mut app = OAuthApp::new();
    let mastodon_endpoint = "http://localhost:3000/api/v1/apps";

    let mut client = herder::api::oauth::make_client(&app, mastodon_endpoint);
    {
        let mut transfer = client.transfer();
        transfer.write_function(|d| {
            app = serde_json::from_slice(d).unwrap();
            Ok(d.len())
        }).unwrap();
        match transfer.perform() {
            Ok(_) => {
                println!("got response");
            },
            _ => println!("couldn't get response"),
        }
    }

    let json = serde_json::to_string(&app).unwrap();
    let mut out = File::create("herder_client.json").unwrap();
    serde_json::to_writer(&mut out, &json).unwrap();
    println!("{}", &json);
}
