use reqwest::blocking::{multipart, Client};
use reqwest::StatusCode;
use serde_json::Value;

use super::utils::{load_test_image, APP_HOST};

pub fn create_test_news(client: &Client) -> Value {
    let buffer = load_test_image();

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

pub fn delete_test_news(client: &Client, news: Value) {
    let response = client
        .delete(format!("{}/news/{}", APP_HOST, news["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
