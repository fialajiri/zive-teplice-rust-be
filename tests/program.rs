use std::fs::File;
use std::io::Read;

use reqwest::StatusCode;
use reqwest::blocking::{multipart, Client};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_program() {
    let client = Client::new();

    let event = common::create_test_event(&client);

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
    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    // Deserialize JSON
    let program: Value = response.json().expect("Failed to parse JSON response");
    println!("create_program response: {:#}", program);
    println!("event: {:#}", event);

    assert_eq!(
        program,
        json!({
            "id": program["id"],
            "event_id": event["id"],
            "image_id": program["image_id"],
            "text": "Some program text",
            "title": "Test Program Title",
            "created_at": program["created_at"],
            "updated_at": program["updated_at"]
        })
    );

    println!("create_program response: {:#}", program);

    common::delete_test_program(&client, program);
    common::delete_test_event(&client, event);
}
