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
    let ojson = r#"{
                    "url": "MYURL",
                    "title": "My Title",
                    "description": "My description",
                    "email": "MYEMAIL"
                 }"#;
    let instance: Instance = serde_json::from_str(ojson).unwrap();
    assert_eq!(instance, Instance {
                        url: String::from("MYURL"),
                        title: String::from("My Title"),
                        description: String::from("My description"),
                        email: String::from("MYEMAIL")
    });
}

#[test]
fn mention_deserialize_from_json() {
    let ojson = r#"{
                    "id": 1234,
                    "url": "MYURL",
                    "username": "MYUSERNAME",
                    "acct": "MYUSERNAME@REMOTEDOMAIN"
                 }"#;
    let mention: Mention = serde_json::from_str(ojson).unwrap();
    assert_eq!(mention, Mention {
                        mention_id: 1234,
                        url: String::from("MYURL"),
                        username: String::from("MYUSERNAME"),
                        acct: String::from("MYUSERNAME@REMOTEDOMAIN")
    });
}

#[test]
fn notification_deserialize_from_json() {
    unimplemented!();
}

#[test]
fn relationship_deserialize_from_json() {
    let ojson = r#"{
                    "id": 1234,
                    "following": true,
                    "followed_by": true,
                    "blocking": true,
                    "muting": true,
                    "requested": true
                 }"#;
    let relationship: Relationship = serde_json::from_str(ojson).unwrap();
    assert_eq!(relationship, Relationship {
                        relationship_id: 1234,
                        following: true,
                        followed_by: true,
                        blocking: true,
                        muting: true,
                        requested: true
    });
}

#[test]
fn report_deserialize_from_json() {
    let ojson = r#"{
                    "id": 1234,
                    "action_taken": "Some action taken"
                 }"#;
    let report: Report = serde_json::from_str(ojson).unwrap();
    assert_eq!(report, Report {
                        report_id: 1234,
                        action_taken: String::from("Some action taken")
    });
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
    let ojson = r#"{
                    "name": "rustlang",
                    "url": "MYURL"
                 }"#;
    let tag: Tag = serde_json::from_str(ojson).unwrap();
    assert_eq!(tag, Tag {
                        name: String::from("rustlang"),
                        url: String::from("MYURL")
    });
}
