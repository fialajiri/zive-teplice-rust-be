use crate::errors::server_error;
use crate::models::program::{NewProgram, UpdateProgram};
use crate::repositories::image::ImageRepository;
use crate::repositories::program::ProgramRepository;
use crate::utils::program_form_config::{ProgramFormConfig, ProgramFormData};

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
    let config = ProgramFormConfig::new();

    let form_data = ProgramFormData::from_multipart(content_type, data, &config).await?;

    println!("{:?}", form_data.text);
    println!("{:?}", form_data.title);
    println!("{:?}", form_data.event_id);

    let image = ImageRepository::new()
        .await
        .map_err(|e| server_error(e.into()))?
        .save_image(&mut db, form_data.image_field)
        .await
        .map_err(|e| server_error(e.into()))?;

    let new_program = NewProgram {
        title: form_data.title,
        text: form_data.text,
        event_id: form_data.event_id,
        image_id: image.id,
    };

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
    let config = ProgramFormConfig::new();

    let form_data = ProgramFormData::from_multipart(content_type, data, &config).await?;

    let repo = ImageRepository::new()
        .await
        .map_err(|e| server_error(e.into()))?;

    // erase the old image

    let image = repo
        .save_image(&mut db, form_data.image_field)
        .await
        .map_err(|e| server_error(e.into()))?;

    let update_program = UpdateProgram {
        title: Some(form_data.title),
        text: Some(form_data.text),
        image_id: Some(image.id),
    };

    ProgramRepository::update(&mut db, id, update_program)
        .await
        .map(|event| json!(event))
        .map_err(|e| server_error(e.into()))
}
