use crate::errors::server_error;
use crate::models::event::NewEvent;
use crate::repositories::event::EventRepository;



use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use super::DbConn;

#[rocket::get("/events/<id>")]
pub async fn get_event<'a>(mut db: Connection<DbConn>, id: i32) -> Result<Custom<Value>, Custom<Value>> {
    EventRepository::find(&mut db, id)
        .await
        .map(|event| Custom(Status::Ok, json!(event)))
        .map_err(|e| server_error(e.into()))
   
}

#[rocket::post("/events", format = "json", data = "<event>")]
pub async fn create_event(
    mut db: Connection<DbConn>,
    event: Json<NewEvent>,  
) -> Result<Custom<Value>, Custom<Value>> {
    EventRepository::create(&mut db, event.into_inner())
        .await
        .map(|event| Custom(Status::Created, json!(event)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/events/<id>")]
pub async fn delete_event<'a>(mut db: Connection<DbConn>, id: i32) -> Result<Custom<Value>, Custom<Value>> {
    EventRepository::delete(&mut db, id)
        .await
        .map(|_| Custom(Status::NoContent, json!("")))
        .map_err(|e| server_error(e.into()))
}

