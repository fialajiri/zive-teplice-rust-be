use crate::errors::server_error;
use crate::models::program::{NewProgram, UpdateProgram};
use crate::repositories::image::ImageRepository;
use crate::repositories::program::ProgramRepository;
use crate::utils::form_data::{FormData, FromFormData};
use crate::utils::form_fields::FormConfig;

use super::DbConn;

use rocket::http::ContentType;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Value};
use rocket::Data;
use rocket_db_pools::Connection;

#[rocket::get("/programs/<event_id>")]
pub async fn get_programs_for_event<'a>(
    mut db: Connection<DbConn>,
    event_id: i32,
) -> Result<Value, Custom<Value>> {
    ProgramRepository::find_program_for_event(&mut db, event_id)
        .await
        .map(|programs| json!(programs))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/programs/<id>")]
pub async fn delete_program<'a>(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    ProgramRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/programs", format = "multipart/form-data", data = "<data>")]
pub async fn create_program<'a>(
    mut db: Connection<DbConn>,
    content_type: &'a ContentType,
    data: Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<NewProgram>::new();

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

    let mut new_program = NewProgram::from_form_data(form_data).unwrap();
    new_program.image_id = image_id.unwrap_or_default();

    ProgramRepository::create_program_for_event(&mut db, new_program)
        .await
        .map(|event| json!(event))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/programs/<id>", format = "multipart/form-data", data = "<data>")]
pub async fn update_program<'a>(
    mut db: Connection<DbConn>,
    id: i32,
    content_type: &'a ContentType,
    data: Data<'a>,
) -> Result<Value, Custom<Value>> {
    let config = FormConfig::<UpdateProgram>::new();
    let form_data = FormData::from_multipart(content_type, data, &config).await?;

    let repo = ImageRepository::new()
        .await
        .map_err(|e| server_error(e.into()))?;

    // erase the old image

    let image_id = if let Some(image_data) = form_data.image_field.clone() {
        let image = repo
            .save_image(&mut db, image_data)
            .await
            .map_err(|e| server_error(e.into()))?;
        Some(image.id)
    } else {
        None
    };

    let mut update_program = UpdateProgram::from_form_data(form_data)?;
    update_program.image_id = image_id;

    ProgramRepository::update(&mut db, id, update_program)
        .await
        .map(|event| json!(event))
        .map_err(|e| server_error(e.into()))
}
