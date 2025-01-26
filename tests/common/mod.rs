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

    let form = multipart::Form::new()
        .text("title", "Test Program Title")
        .text("text", "Some program text")
        .text("event_id", event["id"].as_i64().unwrap().to_string())
        .part("image", part);

    let response = client
        .post(format!("{}/programs", common::APP_HOST))
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

pub fn create_test_news(client: &Client) -> Value {
    let mut file = File::open("./tests/assets/test_image.jpg").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    let form = multipart::Form::new()
        .text("title", "Test News Title")
        .text("message", "This is great news")
        .part("image", part);

    let response = client
        .post(format!("{}/news", APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

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

pub fn delete_test_news(client: &Client, news: Value) {
    let response = client
        .delete(format!("{}/news/{}", APP_HOST, news["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
