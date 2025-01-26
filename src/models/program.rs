use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::programs;
use crate::utils::form_data::FormData;
use crate::utils::form_data::FromFormData;
use crate::utils::form_fields::FormFields;

#[derive(AsChangeset, Queryable, Serialize, Deserialize, Debug)]
pub struct Program {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub text: String,
    pub event_id: i32,
    pub image_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
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

impl FormFields for NewProgram {
    fn get_required_text_fields() -> Vec<&'static str> {
        vec!["title", "text"]
    }
    fn get_required_number_fields() -> Vec<&'static str> {
        vec!["event_id"]
    }
    fn has_image() -> bool {
        true
    }
}

impl FromFormData for NewProgram {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>> {
        Ok(Self {
            title: form_data.required_text_values["title"].clone(),
            text: form_data.required_text_values["text"].clone(),
            event_id: form_data.required_number_values["event_id"],
            image_id: 0, // Will be set after image upload
        })
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = programs)]
pub struct UpdateProgram {
    pub title: Option<String>,
    pub text: Option<String>,
    pub image_id: Option<i32>,
}

impl FormFields for UpdateProgram {
    fn get_optional_text_fields() -> Vec<&'static str> {
        vec!["title", "text"]
    }
    fn has_image() -> bool {
        true
    }
    fn is_image_required() -> bool {
        false
    }
}

impl FromFormData for UpdateProgram {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>> {
        Ok(Self {
            title: form_data
                .optional_text_values
                .get("title")
                .cloned()
                .flatten(),
            text: form_data
                .optional_text_values
                .get("text")
                .cloned()
                .flatten(),
            image_id: None, // Will be set after image upload if present
        })
    }
}
