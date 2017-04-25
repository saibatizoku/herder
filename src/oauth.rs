use curl::easy::{Easy, Form};
use hyper::{Client, Body, Post, Error};
use hyper::client::Request;
use hyper::header::ContentType;
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use url::form_urlencoded;

use std::fmt;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthApp {
    pub id: Option<u64>,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String
}

impl OAuthApp {
    pub fn new() -> OAuthApp {
        OAuthApp {
            id: None,
            client_id: "".to_string(),
            client_secret: "".to_string(),
            redirect_uri: "".to_string()
        }
    }

    pub fn form_data(&self) -> Form {
        let mut form = Form::new();
        form.part("client_name").contents(b"herder").add().unwrap();
        form.part("redirect_uris").contents(b"urn:ietf:wg:oauth:2.0:oob").add().unwrap();
        form.part("scopes").contents(b"read write follow").add().unwrap();
        form
    }
}

impl fmt::Display for OAuthApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "App({:?}, {})", self.id, self.client_id)
    }
}

pub fn make_client(app: &OAuthApp, node_endpoint: &str) -> Easy {
    let mut handle = Easy::new();
    let form_data = app.form_data();
    handle.url(node_endpoint).unwrap();
    handle.httppost(form_data).unwrap();
    handle
}

#[derive(Debug)]
pub struct CreateApp {
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
    pub fn new(name: &str, uris: &str, scopes: &str) -> CreateApp {
        CreateApp {
            client_name: String::from(name),
            redirect_uris: String::from(uris),
            scopes: String::from(scopes)
        }
    }

    pub fn register_app(&self, api_url: &str, dst: Arc<Mutex<Vec<u8>>>) -> Result<(), Error> {
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

    fn form_encode(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("client_name", &self.client_name)
            .append_pair("redirect_uris", &self.redirect_uris)
            .append_pair("scopes", &self.scopes)
            .finish()
    }

}
