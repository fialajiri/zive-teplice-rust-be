use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_event() {
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
fn test_update_event() {
    let client = Client::new();
    let event = common::create_test_event(&client);

    let response = client
        .put(format!("{}/events/{}", common::APP_HOST, event["id"]))
        .json(&json!({
            "id": event["id"],
            "title": "My Updated Event",
            "year": 2026,
            "is_current": false,
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let event: Value = response.json().unwrap();
    assert_eq!(
        event,
        json!({
            "id": event["id"],
            "title": "My Updated Event",
            "year": 2026,
            "is_current": false,
            "created_at": event["created_at"],
            "updated_at": event["updated_at"]
        })
    );

    common::delete_test_event(&client, event);
}

#[test]
fn test_create_event_with_program() {
    let client = Client::new();
    let event = common::create_test_event(&client);
    let program = common::create_test_program_for_event(&client, &event);

    assert_eq!(program["event_id"], event["id"]);

    common::delete_test_program(&client, program);

    common::delete_test_event(&client, event);
}

#[test]
fn test_get_event_with_program() {
    let client = Client::new();
    let (event, program) = common::create_test_event_with_program(&client);

    let response = client
        .get(format!(
            "{}/events/{}/with_program",
            common::APP_HOST,
            event["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let event_with_program: Value = response.json().unwrap();
    assert_eq!(
        event_with_program,
        json!({
            "event_id": event["id"],
            "event_title": event["title"],
            "year": event["year"],
            "is_current": event["is_current"],
            "program_title": program["title"],
            "program_text": program["text"],
            "program_image_id": program["image_id"]

        })
    );

    common::delete_test_program(&client, program);
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
