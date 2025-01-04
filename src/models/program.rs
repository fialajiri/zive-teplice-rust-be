use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::programs;

#[derive(AsChangeset, Queryable, Serialize, Deserialize, Debug)]
pub struct Program {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub text: String,
    pub event_id: i32,
    pub image_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = programs)]
pub struct NewProgram {
    pub title: String,
    pub text: String,
    pub event_id: i32,
    pub image_id: i32,
}
