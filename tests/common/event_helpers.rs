use reqwest::blocking::{multipart, Client};
use reqwest::StatusCode;
use serde_json::{json, Value};

use super::utils::{load_test_image, APP_HOST};

pub fn create_test_event(client: &Client) -> Value {
    let response = client
        .post(format!("{}/events", APP_HOST))
        .json(&json!({
         "title": "My New Event",
         "year": 2025,
         "is_current": true
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn create_test_program_for_event(client: &Client, event: &Value) -> Value {
    let buffer = load_test_image();

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    let form = multipart::Form::new()
        .text("title", "Test Program Title")
        .text("text", "Some program text")
        .text("event_id", event["id"].as_i64().unwrap().to_string())
        .part("image", part);

    let response = client
        .post(format!("{}/programs", APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Failed to create program for supplied event"
    );

    response.json().unwrap()
}

pub fn create_test_event_with_program(client: &Client) -> (Value, Value) {
    let event = create_test_event(client);
    let program = create_test_program_for_event(client, &event);

    (event, program)
}

pub fn delete_test_event(client: &Client, event: Value) {
    let response = client
        .delete(format!("{}/events/{}", APP_HOST, event["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_program(client: &Client, program: Value) {
    let response = client
        .delete(format!("{}/programs/{}", APP_HOST, program["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
