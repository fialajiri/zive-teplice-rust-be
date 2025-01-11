use std::fs::File;
use std::io::Read;

use reqwest::blocking::{multipart, Client};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::common;

pub static APP_HOST: &str = "http://0.0.0.0:8000";

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
    let mut file = File::open("./tests/assets/test_image.jpg").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    // Build multipart form data
    let form = multipart::Form::new()
        .text("title", "Test Program Title")
        .text("text", "Some program text")
        .text("event_id", event["id"].as_i64().unwrap().to_string())
        .part("image", part);

    // Send the request
    let response = client
        .post(format!("{}/programs", common::APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    // Check status code
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
