use crate::errors::server_error;
use crate::models::program::NewProgram;
use crate::repositories::image::ImageRepository;
use crate::repositories::program::ProgramRepository;

use super::DbConn;

use rocket::response::status::{self, Custom};
use rocket::http::ContentType;
use rocket::Data;
use rocket::serde::json::{json, Value};
use rocket_db_pools::Connection;
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};

#[rocket::get("/programs/<event_id>")]
pub async fn get_programs_for_event<'a>(mut db: Connection<DbConn>, event_id: i32) -> Result<Value, Custom<Value>> {
    ProgramRepository::find_programs_for_event(&mut db, event_id).await
        .map(|programs| json!(programs))
        .map_err(|e| server_error(e.into()))
}


#[rocket::post("/programs", format = "multipart/form-data", data = "<data>")]
pub async fn create_program<'a>(mut db: Connection<DbConn>, content_type: &'a ContentType,
    data: Data<'a>) -> Result<Value, Custom<Value>> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw("image")
            .size_limit(30 * 1024 * 1024) // 30 MB
            .content_type_by_string(Some(mime::IMAGE_STAR)).unwrap()
    );
    options.allowed_fields.push(MultipartFormDataField::text("title"));
    options.allowed_fields.push(MultipartFormDataField::text("text"));
    options.allowed_fields.push(MultipartFormDataField::text("event_id"));

    let multipart_form = match MultipartFormData::parse(content_type, data, options).await {
        Ok(form) => form,
        Err(e) => return Err(status::Custom(rocket::http::Status::BadRequest, e.to_string().into())),
    };

    // 2. Extract fields
    let title = match multipart_form.texts.get("title") {
        Some(text_field) => text_field[0].text.clone(),
        None => return Err(status::Custom(rocket::http::Status::BadRequest, "Missing title".into()))
    };
    let text = match multipart_form.texts.get("text") {
        Some(text_field) => text_field[0].text.clone(),
        None => return Err(status::Custom(rocket::http::Status::BadRequest, "Missing text".into()))
    };
    let event_id = match multipart_form.texts.get("event_id") {
        Some(text_field) => text_field[0].text.parse::<i32>().unwrap(),
        None => return Err(status::Custom(rocket::http::Status::BadRequest, "Missing event_id".into()))
    };

    let image_field = match multipart_form.raw.get("image") {
        Some(raw_field_vec) => &raw_field_vec[0],
        None => return Err(status::Custom(rocket::http::Status::BadRequest, "Missing image".into()))
    };

    println!("Title: {}", title);
    println!("text: {}", text);
    

    let image = ImageRepository::save_image(&mut db, image_field).await
        .map_err(|e| server_error(e.into()))?;

    let new_program = NewProgram {
        title,
        text,
        event_id,
        image_id: image.id,
    };

    // 1) Determine the final filename
    // let filename = image_field
    //     .file_name
    //     .clone()
    //     .unwrap_or_else(|| String::from("unnamed"));

    // println!("Filename: {}", filename);

    // let local_path = format!("uploads/{}", filename.to_string());
    // let mut file = File::create(&local_path)
    //     .await
    //     .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e.to_string().into()))?;
    // file.write_all(&image_field.raw)
    //     .await
    //     .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e.to_string().into()))?;

    // let image_id = 215;

    ProgramRepository::create_program_for_event(&mut db, new_program).await
        .map(|event| json!(event))
        .map_err(|e| server_error(e.into()))
}
