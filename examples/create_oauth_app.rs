extern crate herder;
extern crate serde_json;

use herder::Mastodon;

fn main() {
    // FIXME This example needs to point to a Mastodon node in order to run.
    // Need to read it from args to avoid annoying users.
    let api_url = "https://localhost:3000/api/v1/apps";

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
