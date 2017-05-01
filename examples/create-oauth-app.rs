extern crate clap;
extern crate herder;
extern crate serde_json;

use clap::{Arg, App, SubCommand};
use herder::Mastodon;

fn main() {
    let matches = App::new("Herder Mastodon API Client")
        .version("0.1.4")
        .author("saibatizoku")
        .about("Connects to a Mastodon instance")
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .value_name("URL")
             .help("Sets the URL for the Mastodon instance")
             .takes_value(true))
        .get_matches();

    let api_url = matches.value_of("url").unwrap_or("https://localhost:3000/");

    // TODO !!! after successfully getting the herder_app registered,
    // we still need to show the user the link where they can authorize
    // our app. After that, we're good to go using our API!
    //
    // We haven't used herder::Mastodon yet!
    let mastodon = Mastodon(String::from(api_url));
    println!("Mastodon: {:?}", mastodon);
    // use super::oauth::OAuthApp;
    //
    // let oauth_app: OAuthApp = mastodon.create_app("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");
    mastodon.create_app("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");
}
