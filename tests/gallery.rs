use reqwest::blocking::{multipart, Client};
use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_gallery() {
    let client = Client::new();

    let buffer = common::load_test_image();

    let part = multipart::Part::bytes(buffer)
        .file_name("test.image.jpg")
        .mime_str("image/jpeg")
        .expect("Failed to create part");

    let form = multipart::Form::new()
        .text("name", "Test Gallery Name")
        .part("image", part);

    let response = client
        .post(format!("{}/gallery", common::APP_HOST))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    let gallery: Value = response.json().expect("Failed to parse JSON response");

    assert_eq!(
        gallery,
        json!({
            "id": gallery["id"],
            "name": "Test Gallery Name",
            "featured_image_id": gallery["featured_image_id"],
            "created_at": gallery["created_at"],
            "updated_at": gallery["updated_at"]
        })
    );

    common::delete_test_gallery(&client, gallery);
}

#[test]
fn test_get_gallery() {
    let client = Client::new();
    let gallery = common::create_test_gallery(&client);

    let response = client
        .get(format!("{}/gallery/{}", common::APP_HOST, gallery["id"]))
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    let gallery: Value = response.json().expect("Failed to parse JSON response");

    assert_eq!(gallery, gallery);

    common::delete_test_gallery(&client, gallery);
}

#[test]
fn test_get_all_galleries() {
    let client = Client::new();
    let gallery_1 = common::create_test_gallery(&client);
    let gallery_2 = common::create_test_gallery(&client);

    let response = client
        .get(format!("{}/gallery", common::APP_HOST))
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    let galleries: Value = response.json().expect("Failed to parse JSON response");

    assert!(galleries.as_array().unwrap().len() >= 2);

    common::delete_test_gallery(&client, gallery_1);
    common::delete_test_gallery(&client, gallery_2);
}

#[test]
fn test_update_gallery() {
    let client = Client::new();
    let gallery = common::create_test_gallery(&client);

    // change only name
    let form = multipart::Form::new().text("name", "Updated Gallery Name");

    let response = client
        .put(format!("{}/gallery/{}", common::APP_HOST, gallery["id"]))
        .multipart(form)
        .send()
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK, "Response was not 200 OK");

    let gallery: Value = response.json().expect("Failed to parse JSON response");

    assert_eq!(gallery["name"], "Updated Gallery Name");

    common::delete_test_gallery(&client, gallery);
}

#[test]
fn test_delete_gallery() {
    let client = Client::new();
    let gallery = common::create_test_gallery(&client);

    let response = client
        .delete(format!("{}/gallery/{}", common::APP_HOST, gallery["id"]))
        .send()
        .expect("Failed to send request");

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "Response was not 204 NO CONTENT"
    );
}
