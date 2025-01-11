
use crate::errors::server_error;
use crate::repositories::news::NewsRepository;
use rocket_db_pools::Connection;
use super::DbConn;
use rocket::http::ContentType;
use rocket::{response::status::Custom, serde::json::{json, Value}};

#[rocket::get("/news/<id>")]
pub async fn get_news<'a>(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<Value, Custom<Value>> {
    NewsRepository::find(&mut db, id)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/news")]
pub async fn get_all_news<'a>(
    mut db: Connection<DbConn>,
) -> Result<Value, Custom<Value>> {
    NewsRepository::all(&mut db)
        .await
        .map(|news| json!(news))
        .map_err(|e| server_error(e.into()))
}

// #[rocket::post("/news", format = "multipart/form-data", data = "<data>")]
// pub async fn create_news<'a>(
//     mut db: Connection<DbConn>,
//     content_type: &'a ContentType,
//     data: rocket::Data<'a>,
// ) -> Result<Value, Custom<Value>> {
//     let form_data = crate::utils::news_form_config::NewsFormData::from_multipart(data).await?;

//     let new_news = crate::models::news::NewNews {
//         title: form_data.title,
//         message: form_data.message,
//         image_id: form_data.image_id,
//     };

//     NewsRepository::create(&mut db, new_news)
//         .await
//         .map(|news| json!(news))
//         .map_err(|e| server_error(e.into()))
// }

