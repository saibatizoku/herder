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
    let matches = App::new("Herder API Client Creator Example")
        .version("0.1.5")
        .author("saibatizoku")
        .about("Registers a new herder API client on a Mastodon instance, optionally stores results into a JSON file.")
        .arg(Arg::with_name("url")
             .help("Sets the URL, for the Mastodon instance. HTTPS only. Example: https://example.com/")
             .value_name("BASEURL")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("client")
             .help("Specifies the path to load the client app in JSON format. Uses defaults if not present.")
             .short("c")
             .long("client")
             .value_name("CLIENT_JSON_FILE")
             .takes_value(true))
        .arg(Arg::with_name("results")
             .help("Specifies the path to save the JSON results from the server.")
             .value_name("FILE")
             .takes_value(true))
        .get_matches();

    let base_url = matches.value_of("url").unwrap_or("https://localhost:3000");
    let mastodon = Mastodon::new(base_url).chain_err(|| "invalid URL, could not create Mastodon")?;
    let new_app: CreateApp = if matches.is_present("client") {
        let client_path = matches.value_of("client").unwrap();
        let client_json = File::open(client_path).chain_err(|| "Invalid file path")?;
        serde_json::from_reader(client_json).chain_err(|| "Could not save OAuth to JSON File.")?
    } else {
        CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow")
    };
    let oauth_app: OAuthApp = mastodon.register_app(new_app).chain_err(|| "registration of App failed.")?;
    print_success_msg(base_url, &oauth_app);
    if matches.is_present("results") {
        let path = matches.value_of("results").unwrap_or("output.json");
        println!();
        println!("Saving to JSON File: {}", path);
        let mut out = File::create(path).expect("Invalid file path");
        serde_json::to_writer(&mut out, &oauth_app).expect("Could not save OAuth to JSON File.");
        println!("JSON File '{}' saved.", path);
    }
    Ok(())
}

fn print_success_msg(base_url: &str, oauth_app: &OAuthApp) {
    println!();
    println!("Successfully registered our app on {}", base_url);
    println!();
    println!("Visit the following page in order to register this app:");
    println!();
    println!("{}/oauth/authorize?client_id={}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&response_type=code", base_url, oauth_app.client_id);
    println!();
    println!("Once you authorize the app, you will be redirected to a page with your `code`, take note of it! You need it to obtain your access token.");
    println!();
}
