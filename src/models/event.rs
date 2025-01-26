use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::events;

#[derive(AsChangeset, Queryable, Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub is_current: bool,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct EventWithProgram {
    pub event_id: i32,
    pub event_title: String,
    pub year: i32,
    pub is_current: bool,
    pub program_title: Option<String>,
    pub program_text: Option<String>,
    pub program_image_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub year: i32,
    pub is_current: bool,
}
