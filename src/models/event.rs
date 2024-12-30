use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::events;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub is_current: bool,
    pub program_title: Option<String>,
    pub program_text: Option<String>,
    pub image_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub year: i32,
    pub is_current: bool,
    pub program_title: Option<String>,
    pub program_text: Option<String>,
    pub image_id: i32,
}