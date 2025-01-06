
use crate::errors::server_error;
use crate::repositories::news::NewsRepository;
use rocket_db_pools::Connection;
use super::DbConn;
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
