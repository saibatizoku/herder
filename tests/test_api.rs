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

fn mock_account() -> Account {
    Account {
        account_id: 1234,
        username: String::from("MYUSERNAME"),
        acct: String::from("MYUSERNAME@MYREMOTEDOMAIN"),
        display_name: String::from("jane_sanchez"),
        note: String::from("A self-described person"),
        url: String::from("MYRUL"),
        avatar: String::from("MYURL.png"),
        header: String::from("MYHEADER.png"),
        locked: false,
        created_at: String::from("2000-01-01T00:00:00Z"),
        followers_count: 1234,
        following_count: 1234,
        statuses_count: 1234
    }
}

fn mock_application() -> Application {
    Application {
        name: String::from("MYNAME"),
        website: Some(String::from("MYURI"))
    }
}

fn mock_status() -> Status {
    Status {
        status_id: 1234,
        uri: String::from("MYURI"),
        url: String::from("MYURL"),
        account: mock_account(),
        in_reply_to_id: Some(1234),
        in_reply_to_account_id: Some(1234),
        reblog: None,
        content: String::from("My tooted toot!"),
        created_at: String::from("2000-01-01T00:00:00Z"),
        reblogs_count: 555,
        favourites_count: 777,
        reblogged: true,
        favourited: true,
        sensitive: false,
        spoiler_text: String::from(""),
        visibility: String::from(""),
        media_attachments: Vec::new(),
        mentions: Vec::new(),
        tags: Vec::new(),
        application: mock_application()
    }
}

#[test]
fn account_deserialize_from_json() {
    let ojson = r#"{
        "id": 1234,
        "username": "MYUSERNAME",
        "acct": "MYUSERNAME@MYREMOTEDOMAIN",
        "display_name": "jane_sanchez",
        "note": "A self-described person",
        "url": "MYRUL",
        "avatar": "MYURL.png",
        "header": "MYHEADER.png",
        "locked": false,
        "created_at": "2000-01-01T00:00:00Z",
        "followers_count": 1234,
        "following_count": 1234,
        "statuses_count": 1234
    }"#;
    let account: Account = serde_json::from_str(ojson).unwrap();
    assert_eq!(account, mock_account());
}

#[test]
fn application_deserialize_from_json() {
    let ojson = r#"{
                    "name": "MYNAME",
                    "website": "MYURI"
                 }"#;
    let application: Application = serde_json::from_str(ojson).unwrap();
    assert_eq!(application, mock_application());
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
    let ojson = r#"{
                    "ancestors": [],
                    "descendants": []
                 }"#;
    let context: Context = serde_json::from_str(ojson).unwrap();
    assert_eq!(context, Context {
        ancestors: Vec::new(),
        descendants: Vec::new()
    });
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
    let ojson = r#"{
                    "id": 1234,
                    "type": "NOTIFICATIONTYPE",
                    "created_at": "2000-01-01T00:00:00Z",
                    "account": {
                        "id": 1234,
                        "username": "MYUSERNAME",
                        "acct": "MYUSERNAME@MYREMOTEDOMAIN",
                        "display_name": "jane_sanchez",
                        "note": "A self-described person",
                        "url": "MYRUL",
                        "avatar": "MYURL.png",
                        "header": "MYHEADER.png",
                        "locked": false,
                        "created_at": "2000-01-01T00:00:00Z",
                        "followers_count": 1234,
                        "following_count": 1234,
                        "statuses_count": 1234
                    },
                    "status": null
                 }"#;
    let notification: Notification = serde_json::from_str(ojson).unwrap();
    assert_eq!(notification, Notification {
        notification_id: 1234,
        notification_type: String::from("NOTIFICATIONTYPE"),
        created_at: String::from("2000-01-01T00:00:00Z"),
        account: mock_account(),
        status: None
    });
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
    let ojson = r#"{
                    "accounts": [],
                    "statuses": [],
                    "hashtags": []
                 }"#;
    let results: Results = serde_json::from_str(ojson).unwrap();
    assert_eq!(results, Results {
        accounts: Vec::new(),
        statuses: Vec::new(),
        hashtags: Vec::new()
    });
}

#[test]
fn status_deserialize_from_json() {
    let ojson = r#"{
                    "id": 1234,
                    "uri": "MYURI",
                    "url": "MYURL",
                    "account": {
                        "id": 1234,
                        "username": "MYUSERNAME",
                        "acct": "MYUSERNAME@MYREMOTEDOMAIN",
                        "display_name": "jane_sanchez",
                        "note": "A self-described person",
                        "url": "MYRUL",
                        "avatar": "MYURL.png",
                        "header": "MYHEADER.png",
                        "locked": false,
                        "created_at": "2000-01-01T00:00:00Z",
                        "followers_count": 1234,
                        "following_count": 1234,
                        "statuses_count": 1234
                    },
                    "in_reply_to_id": 1234,
                    "in_reply_to_account_id": 1234,
                    "reblog": null,
                    "content": "My tooted toot!",
                    "created_at": "2000-01-01T00:00:00Z",
                    "reblogs_count": 555,
                    "favourites_count": 777,
                    "reblogged": true,
                    "favourited": true,
                    "sensitive": false,
                    "spoiler_text": "",
                    "visibility": "",
                    "media_attachments": [],
                    "mentions": [],
                    "tags": [],
                    "application": {
                        "name": "MYNAME",
                        "website": "MYURI"
                    }
                 }"#;
    let status: Status = serde_json::from_str(ojson).unwrap();
    assert_eq!(status, mock_status());
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
