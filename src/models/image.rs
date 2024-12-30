use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::images;



#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: i32,
    pub image_url: String,
    pub image_key: String,
    pub width: i32,
    pub height: i32,   
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub image_url: String,
    pub image_key: String,
    pub width: i32,
    pub height: i32,
}