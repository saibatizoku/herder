extern crate herder;
extern crate serde_json;

use herder::Mastodon;
use herder::oauth::{CreateApp, OAuthApp};

use std::sync::{Arc, Mutex};

fn main() {
    // # Ways to create an app
    //
    // let social_app = CreateApp {
    //     client_name: String::from("herder"),
    //     redirect_uris: String::from("urn:ietf:wg:oauth:2.0:oob"),
    //     scopes: String::from("read write follow")
    // };
    //
    // let social_app = CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");
    //
    // let social_app = CreateApp::default();
    //
    let api_url = "https://mastodon.cloud/api/v1/apps";
    let out = Arc::new(Mutex::new(Vec::new()));

    let social_app = CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");

    {
        social_app.register_app(api_url, out.clone()).unwrap();

        let result = out.lock().unwrap();
        let herder_app: OAuthApp = serde_json::from_slice(&result).unwrap();

        println!("\n{:?}\n", &herder_app);
        //println!("\n{}\n", serde_json::to_string(&herder_app).unwrap());
    }

    // TODO !!! after successfully getting the herder_app registered,
    // we still need to show the user the link where they can authorize
    // our app. After that, we're good to go using our API!
    //
    // We haven't used herder::Mastodon yet!
    let mastodon = Mastodon(String::from(api_url));
    println!("Mastodon: {:?}", mastodon);
}

