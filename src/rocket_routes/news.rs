use super::DbConn;
use crate::errors::server_error;
use crate::models::news::{NewNews, UpdateNews};
use crate::repositories::image::ImageRepository;
use crate::repositories::news::NewsRepository;
use crate::utils::form_data::{FormData, FromFormData};
use crate::utils::form_fields::FormConfig;
use rocket::http::ContentType;
use rocket::{
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/news/<id>")]
pub async fn get_news<'a>(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    NewsRepository::find(&mut db, id)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/news")]
pub async fn get_all_news<'a>(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    NewsRepository::all(&mut db)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/news", format = "multipart/form-data", data = "<data>")]
pub async fn create_news<'a>(
    mut db: Connection<DbConn>,
    content_type: &'a ContentType,
    data: rocket::Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<NewNews>::new();
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

    let mut new_news = NewNews::from_form_data(form_data).unwrap();
    new_news.image_id = image_id.unwrap_or_default();

    NewsRepository::create(&mut db, new_news)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/news/<id>", format = "multipart/form-data", data = "<data>")]
pub async fn update_news<'a>(
    mut db: Connection<DbConn>,
    id: i32,
    content_type: &'a ContentType,
    data: rocket::Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<UpdateNews>::new();
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

    let mut update_news = UpdateNews::from_form_data(form_data).unwrap();

    if let Some(image_id) = image_id {
        update_news.image_id = Some(image_id);
    }

    println!("{:?}", update_news);

    NewsRepository::update(&mut db, id, update_news)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/news/<id>")]
pub async fn delete_news<'a>(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<rocket::response::status::NoContent, Custom<Value>> {
    NewsRepository::delete(&mut db, id)
        .await
        .map(|_| rocket::response::status::NoContent)
        .map_err(|e| server_error(e.into()))
}
