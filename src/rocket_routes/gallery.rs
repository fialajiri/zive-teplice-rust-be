use super::DbConn;
use crate::errors::server_error;
use crate::models::gallery::{NewGallery, UpdateGallery};
use crate::repositories::gallery::GalleryRepository;
use crate::repositories::image::ImageRepository;
use crate::utils::form_data::{FormData, FromFormData};
use crate::utils::form_fields::FormConfig;
use rocket::http::ContentType;
use rocket::{
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/gallery/<id>")]
pub async fn get_gallery<'a>(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    GalleryRepository::find(&mut db, id)
        .await
        .map(|gallery| json!(gallery))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/gallery")]
pub async fn get_all_galleries<'a>(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    GalleryRepository::all(&mut db)
        .await
        .map(|galleries| json!(galleries))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/gallery", format = "multipart/form-data", data = "<data>")]
pub async fn create_gallery<'a>(
    mut db: Connection<DbConn>,
    content_type: &'a ContentType,
    data: rocket::Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<NewGallery>::new();
    let form_data = FormData::from_multipart(content_type, data, &config).await?;

    let repo = ImageRepository::new()
        .await
        .map_err(|e| server_error(e.into()))?;

    let image_id = if let Some(image_data) = form_data.image_field.clone() {
        let image = repo
            .save_image(&mut db, image_data)
            .await
            .map_err(|e| server_error(e.into()))?;
        Some(image.id)
    } else {
        None
    };

    let mut new_gallery = NewGallery::from_form_data(form_data).unwrap();
    new_gallery.featured_image_id = image_id.unwrap_or_default();

    GalleryRepository::create(&mut db, new_gallery)
        .await
        .map(|gallery| json!(gallery))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/gallery/<id>", format = "multipart/form-data", data = "<data>")]
pub async fn update_gallery<'a>(
    mut db: Connection<DbConn>,
    id: i32,
    content_type: &'a ContentType,
    data: rocket::Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<UpdateGallery>::new();
    let form_data = FormData::from_multipart(content_type, data, &config).await?;

    let repo = ImageRepository::new()
        .await
        .map_err(|e| server_error(e.into()))?;

    let image_id = if let Some(image_data) = form_data.image_field.clone() {
        let image = repo
            .save_image(&mut db, image_data)
            .await
            .map_err(|e| server_error(e.into()))?;
        Some(image.id)
    } else {
        None
    };

    let mut update_gallery = UpdateGallery::from_form_data(form_data).unwrap();

    if let Some(image_id) = image_id {
        update_gallery.featured_image_id = Some(image_id);
    }

    GalleryRepository::update(&mut db, id, update_gallery)
        .await
        .map(|gallery| json!(gallery))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/gallery/<id>")]
pub async fn delete_gallery<'a>(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<rocket::response::status::NoContent, Custom<Value>> {
    GalleryRepository::delete(&mut db, id)
        .await
        .map(|_| rocket::response::status::NoContent)
        .map_err(|e| server_error(e.into()))
}
