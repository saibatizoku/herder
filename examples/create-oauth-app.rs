#![recursion_limit = "1024"]
extern crate clap;
extern crate herder;
extern crate serde_json;

use clap::{Arg, App};
use herder::Mastodon;
use herder::errors::*;
use herder::mastodon::NodeInstance;
use herder::api::oauth::{CreateApp, OAuthApp};

use std::fs::File;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = App::new("Herder Mastodon API Client")
        .version("0.1.5")
        .author("saibatizoku")
        .about("Connects to a Mastodon instance")
        .arg(Arg::with_name("url")
             .help("Sets the URL, https only, for the Mastodon instance. Example: https://example.com/")
             .value_name("BASEURL")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("json")
             .help("Specifies the path to save the JSON results from the server, when successful.")
             .short("j")
             .long("json-file")
             .value_name("FILE")
             .takes_value(true))
        .get_matches();

    let base_url = matches.value_of("url").unwrap_or("https://localhost:3000");
    let mastodon = Mastodon::new(base_url).unwrap();
    let oauth_app: OAuthApp = mastodon.register_app(CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow")).unwrap();

    println!("Run the following with `curl`:");
    println!();
    println!("curl -X POST -d 'client_id={}&client_secret={}&grant_type=password&username=YOUR_EMAIL&password=YOUR_PASSWORD' -Ss  {}/oauth/token", oauth_app.client_id, oauth_app.client_secret, base_url);
    println!();
    println!("to register this app on your Mastodon node. Replace `YOUR_EMAIL` and `YOUR_PASSWORD` with the appropriate values.");
    println!();
    println!("NOTE that your login credentials are being sent. This is not considered the best practice. Be careful and be warned!");

    if matches.is_present("json") {
        let path = matches.value_of("json").unwrap_or("output.json");
        println!();
        println!("Saving to JSON File: {}", path);
        let mut out = File::create(path).expect("Invalid file path");
        serde_json::to_writer(&mut out, &oauth_app).expect("Could not save OAuth to JSON File.");
        println!("JSON File '{}' saved.", path);
    }
    Ok(())
}
