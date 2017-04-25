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
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use url::form_urlencoded;

use std::io::Write;
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
    let api_url = "http://localhost:3000/api/v1/apps";
    let out = Arc::new(Mutex::new(Vec::new()));

    let social_app = CreateApp::new("herder-app", "urn:ietf:wg:oauth:2.0:oob", "read write follow");

    social_app.register_app(api_url, out.clone()).unwrap();

    let result = out.lock().unwrap();
    let herder_app: OAuthApp = serde_json::from_slice(&result).unwrap();

    println!("\n{:?}\n", &herder_app);
    //println!("\n{}\n", serde_json::to_string(&herder_app).unwrap());
    //
    // TODO !!! after successfully getting the herder_app registered,
    // we still need to show the user the link where they can authorize
    // our app. After that, we're good to go using our API!
    //
    // We haven't used herder::Mastodon yet!
    let mastodon = Mastodon(String::from(api_url));
    println!("Mastodon: {:?}", mastodon);
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

    fn form_encode(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("client_name", &self.client_name)
            .append_pair("redirect_uris", &self.redirect_uris)
            .append_pair("scopes", &self.scopes)
            .finish()
    }

    fn register_app(&self, api_url: &str, dst: Arc<Mutex<Vec<u8>>>) -> Result<(), hyper::Error> {
        let mut core = Core::new().unwrap();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()))
            .build(&core.handle());

        let mut req: Request<Body> = Request::new(Post, api_url.parse().unwrap());
        req.headers_mut().set(ContentType::form_url_encoded());
        req.set_body(Body::from(self.form_encode()));
        let mut dst = dst.lock().unwrap();
        let work = client.request(req).and_then(|res| {
            res.body().for_each(|chunk| {
                dst.extend_from_slice(&chunk);
                ::std::io::stdout().write_all(&chunk)
                    .map(|_| ())
                    .map_err(From::from)
            })
        });
        core.run(work).expect("couldn't work");
        Ok(())
    }
}
