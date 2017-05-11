extern crate clap;
extern crate herder;
extern crate serde_json;

use clap::{Arg, App};
use herder::Mastodon;
use herder::oauth::OAuthApp;

fn main() {
    let matches = App::new("Herder Mastodon API Client")
        .version("0.1.4")
        .author("saibatizoku")
        .about("Connects to a Mastodon instance")
        .arg(Arg::with_name("url")
             .help("Sets the URL, https only, for the Mastodon instance. Example: https://example.com/")
             .value_name("BASEURL")
             .required(true)
             .takes_value(true))
        .get_matches();

    let base_url = matches.value_of("url").unwrap_or("https://localhost:3000");
    let mastodon = Mastodon(format!("{}", base_url));
    let oauth_app: OAuthApp = mastodon.register_app("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");

    println!("Run the following with `curl`:");
    println!();
    println!("curl -X POST -d 'client_id={}&client_secret={}&grant_type=password&username=YOUR_EMAIL&password=YOUR_PASSWORD' -Ss  {}/oauth/token", oauth_app.client_id, oauth_app.client_secret, base_url);
    println!("\nto register this app on your Mastodon node. Replace `YOUR_EMAIL` and `YOUR_PASSWORD` with the appropriate values.");
    println!("\nNOTE that your login credentials are being sent. This is not considered the best practice. Be careful and be warned!");
}
