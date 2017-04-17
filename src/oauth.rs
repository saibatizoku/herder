use curl::easy::{Easy, Form};
use std::fmt;

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
