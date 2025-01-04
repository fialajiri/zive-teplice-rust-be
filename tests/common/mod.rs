use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json, Value};

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