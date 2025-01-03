use crate::errors::server_error;
use crate::models::event::NewEvent;
use crate::repositories::event::EventRepository;



use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use super::DbConn;

#[rocket::get("/events")]
pub async fn get_events<'a>(_db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
   Ok(json!("Hello"))
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

