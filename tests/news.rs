use std::fs::File;
use std::io::Read;

use reqwest::blocking::{multipart, Client};
use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_news() {
    let client = Client::new();

    let mut file = File::open("./tests/assets/test_image.jpg").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    // Build multipart form data
    let form = multipart::Form::new()
        .text("title", "Test News Title")
        .text("message", "This is great news")
        .part("image", part);

    // Send the request
    let response = client
        .post(format!("{}/news", common::APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    // Check status code
    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    // Deserialize JSON
    let news: Value = response.json().expect("Failed to parse JSON response");

    assert_eq!(
        news,
        json!({
            "id": news["id"],
            "image_id": news["image_id"],
            "title": "Test News Title",
            "message": "This is great news",
            "created_at": news["created_at"],
            "updated_at": news["updated_at"]
        })
    );

    common::delete_test_news(&client, news);
}
