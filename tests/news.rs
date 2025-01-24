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

    let form = multipart::Form::new()
        .text("title", "Test News Title")
        .text("message", "This is great news")
        .part("image", part);

    let response = client
        .post(format!("{}/news", common::APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

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

#[test]
fn test_get_single_news() {
    let client = Client::new();
    let news = common::create_test_news(&client);

    let response = client
        .get(format!("{}/news/{}", common::APP_HOST, news["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let news: Value = response.json().unwrap();

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

#[test]
fn test_get_all_news() {
    let client = Client::new();
    let news_one = common::create_test_news(&client);
    let news_two = common::create_test_news(&client);

    let response = client
        .get(format!("{}/news", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let news: Value = response.json().unwrap();

    assert!(news.as_array().unwrap().len() >= 2);

    common::delete_test_news(&client, news_one);
    common::delete_test_news(&client, news_two);
}

#[test]
fn test_update_news() {
    let client = Client::new();
    let news = common::create_test_news(&client);

    // change only title
    let form = multipart::Form::new().text("title", "Updated News Title");

    let response = client
        .put(format!("{}/news/{}", common::APP_HOST, news["id"]))
        .multipart(form)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let old_news: Value = response.json().unwrap();

    assert_eq!(
        old_news,
        json!({
            "id": news["id"],
            "image_id": news["image_id"],
            "title": "Updated News Title",
            "message": "This is great news",
            "created_at": news["created_at"],
            "updated_at": news["updated_at"]
        })
    );

    // change only image
    let mut file = File::open("./tests/assets/test_image.jpg").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    let form = multipart::Form::new().part("image", part);

    let response = client
        .put(format!("{}/news/{}", common::APP_HOST, news["id"]))
        .multipart(form)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let news: Value = response.json().unwrap();

    // assert that the image_id has changed
    assert_ne!(old_news["image_id"], news["image_id"]);

    common::delete_test_news(&client, news);
}

#[test]
fn test_delete_news() {
    let client = Client::new();
    let news = common::create_test_news(&client);

    let response = client
        .delete(format!("{}/news/{}", common::APP_HOST, news["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
