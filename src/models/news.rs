use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::news;
use crate::utils::form_data::{FormData, FromFormData};
use crate::utils::form_fields::FormFields;

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

impl FormFields for NewNews {
    fn get_required_text_fields() -> Vec<&'static str> {
        vec!["title", "message"]
    }
    fn has_image() -> bool {
        true
    }
}

impl FromFormData for NewNews {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>> {
        Ok(Self {
            title: form_data.required_text_values["title"].clone(),
            message: form_data.required_text_values["message"].clone(),
            image_id: 0, // Will be set after image upload
        })
    }
}
