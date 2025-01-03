use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::events;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub is_current: bool, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub year: i32,
    pub is_current: bool,   
}