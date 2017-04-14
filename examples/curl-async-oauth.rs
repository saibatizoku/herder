extern crate curl;
extern crate futures;
extern crate herder;
extern crate tokio_core;
extern crate tokio_curl;
extern crate serde_json;

use std::str;
use std::sync::{Arc, Mutex};

use curl::easy::{Easy, Form};
use futures::{done, Future};
use herder::oauth::OAuthApp;
use tokio_core::reactor::Core;
use tokio_curl::Session;


fn main() {
    // Top-level bindings
    //
    // mastodon_endpoint is used to specify the URL for registering new
    // OAuth Apps on the Mastodon Node.
    let mastodon_endpoint = "http://localhost:3000/api/v1/apps";
    let out = Arc::new(Mutex::new(Vec::new()));

    // tokio-related bindings
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());

    // curl-related bindings
    let mut req = Easy::new();
    let mut form_data = Form::new();

    // Add form-data about our OAuth App
    form_data.part("client_name").contents(b"herder").add().unwrap();
    form_data.part("redirect_uris").contents(b"urn:ietf:wg:oauth:2.0:oob").add().unwrap();
    form_data.part("scopes").contents(b"read write follow").add().unwrap();

    // Set URL
    req.url(&mastodon_endpoint).unwrap();
    req.httppost(form_data).unwrap();

    let dst = out.clone();
    req.write_function(move |data| {
            let mut dst = dst.lock().unwrap();
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();

    let request = session.perform(req).then(|_| {
        let result = out.lock().unwrap();
		let myapp: OAuthApp = serde_json::from_slice(&result).unwrap();
        println!("len: {} out:\n{}",
                 result.len(),
                 serde_json::to_string(&myapp).unwrap());
        done::<u32, u32>(Ok(1))
    });

    // execute and wait result
    lp.run(request).unwrap();
}
