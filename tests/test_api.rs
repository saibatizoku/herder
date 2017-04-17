extern crate herder;
extern crate serde_json;

use herder::api;

#[test]
fn account_from_json() {
    let ojson = r#"{
                    "name": "MYNAME",
                    "website": "MYURI"
                 }"#;
    let application: api::v1::Application = serde_json::from_str(ojson).unwrap();
    assert_eq!(application, api::v1::Application {
                        name: String::from("MYNAME"),
                        website: Some(String::from("MYURI"))
    });
}
