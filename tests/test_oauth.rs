extern crate herder;
extern crate serde_json;

use herder::OAuthApp;

#[test]
fn oauthapp_from_json() {
    let ojson = "{\"id\":1234,\"client_id\":\"CID\",\"client_secret\":\"CSEC\",\"redirect_uri\":\"MYURI\"}".as_bytes();
    let oauth: OAuthApp = serde_json::from_slice(&ojson).unwrap();
    assert_eq!(oauth, OAuthApp {
                        id: Some(1234),
                        client_id: String::from("CID"),
                        client_secret: String::from("CSEC"),
                        redirect_uri: String::from("MYURI")
    });
}
