#![recursion_limit = "1024"]
extern crate clap;
extern crate futures;
extern crate herder;
extern crate hyper;
extern crate hyper_tls;
extern crate rpassword;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate url;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};
use herder::api::oauth::OAuthApp;
use herder::errors::*;
use hyper::Client as WebClient;
use hyper::{Body, Post, Uri};
use hyper::client::{FutureResponse, Request, Response};
use hyper::header::{Headers, ContentType, Authorization, Bearer};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use futures::future::AndThen;
use tokio_core::reactor::Core;
use url::Url;

use std::io;
use std::io::Write;
use std::fs::File;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub enum AuthorizationType {
    ClientCredentials {
        username: String
    },
    Password {
        user: String,
        pass: String
    },
    RequestAuthorization { user: String }
}

struct HerderData {
    data: Arc<Mutex<Vec<u8>>>
}

impl HerderData {
    fn new() -> Self {
        HerderData { data: Arc::new(Mutex::new(Vec::new())) }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct APIToken {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: i64
}

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
    let matches = App::new("Herder Mastodon API Token Retriever")
        .version("0.1.5")
        .author("saibatizoku")
        .about("Retrieves a bearer token for a given set of Client credentials.")
        .arg(Arg::with_name("url")
             .help("Sets the URL, https only, for the Mastodon instance. Example: https://example.com/")
             .required(true)
             .takes_value(true)
             .value_name("BASE_URL"))
        .arg(Arg::with_name("client")
             .help("Specifies the path to save the JSON results from the server, when successful. Defaults to: ./client.json")
             .required(true)
             .takes_value(true)
             .value_name("CLIENT_JSON_FILE"))
        .get_matches();
    println!("Retrieving bearer token...");
    println!("\tReading input file...");
    let base_url = matches.value_of("url").unwrap_or("https://localhost:3000");
    let client_path = matches.value_of("client").unwrap_or("client.json");

    let base_url = Url::from_str(base_url).chain_err(|| "Invalid URL")?;
    let end_url = base_url.join("/oauth/token")?;
    let mastodon_uri = Uri::from_str(end_url.as_str()).chain_err(|| "Invalid URL")?;

    println!("\t\tURL: {}", base_url);
    println!("\t\tURI: {}", mastodon_uri);
    println!("\t\tClient: {}", client_path);

    let client_json = File::open(client_path).chain_err(|| "Invalid file path")?;
    let app: OAuthApp = serde_json::from_reader(client_json).chain_err(|| "Could not save OAuth to JSON File.")?;
    println!("\t...Loaded app: {}", app);
    println!();
    println!("\tUser credentials:");
    {
        println!("username:");
        let mut user = String::new();
        io::stdin().read_line(&mut user).chain_err(|| "wrong username").unwrap();

        //let mut pass = rpassword::prompt_password_stdout("password?")
        //    .chain_err(|| "Could not process password").unwrap();

        user.pop();
        //pass.pop();

        let herder_data = HerderData::new();
        let data = herder_data.data;

        fetch_token(mastodon_uri.as_ref(), &app, AuthorizationType::RequestAuthorization{ user }, data.clone())?;

        {
            let data = data.lock().unwrap();
            if data.is_empty() { panic!("Invalid result. Empty") }
            println!();
            io::stdout().write_all(&data)?;
            println!();
            let api_token: APIToken = serde_json::from_slice(&data).chain_err(|| "Unexpected JSON error.")?;
            println!("Token: {:?}", api_token);
            println!();
        }
    }
    Ok(())
}
fn fetch_token(url: &str, app: &OAuthApp, grant_type: AuthorizationType, data: Arc<Mutex<Vec<u8>>>) -> Result<()> {
    let mut core = Core::new().expect("Could not start client reactor");
    let client = WebClient::configure()
        .connector(HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let mut uri = Url::from_str(url)?;
    // uri.query_pairs_mut().append_pair("id[]", "43");
    // uri.query_pairs_mut().append_pair("id[]", "44");
    // println!("query: {}", &uri.query().unwrap());

    //let params = client_credentials_grant_type(app);
    //let body_str = serde_urlencoded::to_string(params).unwrap();
    let body_str = match grant_type {
        AuthorizationType::ClientCredentials { username: user } => {
            let params = &[
                ("client_id", &app.client_id),
                ("client_secret", &app.client_secret),
                ("grant_type", &String::from("client_credentials")),
                ("scope", &"read write follow".to_owned())
                    ];
            serde_urlencoded::to_string(params).unwrap()
        },
        AuthorizationType::Password { user, pass } => {
            let params = &[
                ("client_id", &app.client_id),
                ("client_secret", &app.client_secret),
                ("grant_type", &"password".to_owned()),
                ("username", &user),
                ("password", &pass)
                    ];
            serde_urlencoded::to_string(params).unwrap()
        },
        AuthorizationType::RequestAuthorization { user } => {
            let params = &[
                ("client_id", &app.client_id),
                ("client_secret", &app.client_secret),
                ("redirect_uri", &app.redirect_uri),
                ("code", &"CODE".to_owned()),
                ("grant_type", &"authorization_code".to_owned())
                    ];
            serde_urlencoded::to_string(params).unwrap()
        }
    };
    let req = build_request(uri.as_str(), None, Some(&body_str))?;
    println!("Created a new request: {:#?}", &req);
    //println!("Created a herder response: {}", data.lock().unwrap());
    //println!("Request: {:#?}", &req);
    //println!("Body: {:#?}", &req.body());
    println!("Body text: {:?}", body_str);
    let mut data = data.lock().unwrap();
    let work = client.request(req).and_then(|res: Response| {
        println!("response {:#?}", res);
        res.body().for_each(|chunk| {
            data.extend_from_slice(&chunk);
            Ok(())
        })
    });
    core.run(work).chain_err(|| "Failed to run registration")?;
    Ok(())
}

fn build_request(url: &str, query: Option<&str>, body: Option<&str>) -> Result<Request<Body>> {
    let uri = Uri::from_str(url).chain_err(|| "Invalid URI for endpoint")?;
    let mut req: Request<Body> = Request::new(Post, uri);
    let mut headers = Headers::new();
    //headers.set(
    //    Authorization(
    //        Bearer {
    //            token: "4a96540a231038a1346601e01eecfef36aece0e181a1362037b3a284db731246".to_owned()
    //        }
    //    )
    //);
    if body.is_some() {
        headers.set(ContentType::form_url_encoded());
        req.set_body(Body::from(body.unwrap().to_owned()));
    }
    req.headers_mut().clone_from(&headers);

    Ok(req)
}
