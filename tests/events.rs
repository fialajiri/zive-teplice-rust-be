use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();
    

    let response = client
    .post(format!("{}/events", common::APP_HOST))
    .json(&json!({            
        "title": "My New Event",
        "year": 2025,
        "is_current": true
       }))
    .send()
    .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    common::delete_test_event(&client, response.json().unwrap());
}

#[test]
fn test_get_event() {
    let client = Client::new();
    let event = common::create_test_event(&client);

    let response = client
        .get(format!("{}/events/{}", common::APP_HOST, event["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let event: Value = response.json().unwrap();
    assert_eq!(
        event,
        json!({
            "id": event["id"],
            "title": "My New Event",
            "year": 2025,
            "is_current": true,
            "created_at": event["created_at"],
            "updated_at": event["updated_at"]
        })
    );

    common::delete_test_event(&client, event);


}

#[test]
fn test_delete_event() {
    let client = Client::new();
    let event = common::create_test_event(&client);

    let response = client
        .delete(format!("{}/events/{}", common::APP_HOST, event["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}