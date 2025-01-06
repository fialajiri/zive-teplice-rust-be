use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::news;



#[derive(AsChangeset, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = news)]
pub struct News {
    pub id: i32,
    pub title: String,
    pub message: String,
    pub image_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = news)]
pub struct NewNews {
    pub title: String,
    pub message: String,
    pub image_id: i32,
}