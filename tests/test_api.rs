extern crate herder;
extern crate serde_json;

use herder::api::v1::{
    Account,
    Application,
    Attachment,
    Card,
    Context,
    Error,
    Instance,
    Mention,
    Notification,
    Relationship,
    Report,
    Results,
    Status,
    Tag
};

#[test]
fn account_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn application_deserialize_from_json() {
    let ojson = r#"{
                    "name": "MYNAME",
                    "website": "MYURI"
                 }"#;
    let application: Application = serde_json::from_str(ojson).unwrap();
    assert_eq!(application, Application {
                        name: String::from("MYNAME"),
                        website: Some(String::from("MYURI"))
    });
}

#[test]
fn attachment_deserialize_from_json() {
    let ojson = r#"{
                    "id": 1234,
                    "type": "video",
                    "url": "MYURL",
                    "remote_url": "MYURL",
                    "preview_url": "MYURL",
                    "text_url": "MYURL"
                 }"#;
    let attachment: Attachment = serde_json::from_str(ojson).unwrap();
    assert_eq!(attachment, Attachment {
                        attachment_id: 1234,
                        attachment_type: String::from("video"),
                        url: String::from("MYURL"),
                        remote_url: String::from("MYURL"),
                        preview_url: String::from("MYURL"),
                        text_url: String::from("MYURL")
    });
}

#[test]
fn card_deserialize_from_json() {
    let ojson = r#"{
                    "url": "MYURL",
                    "title": "My Title",
                    "description": "My description",
                    "image": "MYIMG.png"
                 }"#;
    let card: Card = serde_json::from_str(ojson).unwrap();
    assert_eq!(card, Card {
                        url: String::from("MYURL"),
                        title: String::from("My Title"),
                        description: String::from("My description"),
                        image: String::from("MYIMG.png")
    });
}

#[test]
fn context_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn error_deserialize_from_json() {
    let ojson = r#"{
                    "error": "Error message"
                 }"#;
    let error: Error = serde_json::from_str(ojson).unwrap();
    assert_eq!(error, Error {
                        error: String::from("Error message")
    });
}

#[test]
fn instance_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn mention_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn notification_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn relationship_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn report_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn results_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn status_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn tag_deserialize_from_json() {
    unimplemented!();
}
