use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;
use rocket::response::status::Custom;
use rocket::serde::json::Value;

use crate::models::image::Image;
use crate::schema::{galleries, gallery_images};
use crate::utils::form_data::{FormData, FromFormData};
use crate::utils::form_fields::FormFields;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = galleries)]
pub struct Gallery {
    pub id: i32,
    pub name: String,  
    pub featured_image_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = galleries)]
pub struct NewGallery {
    pub name: String,   
    pub featured_image_id: i32,
}

impl FormFields for NewGallery {
    fn get_required_text_fields() -> Vec<&'static str> {
        vec!["name"]
    }
    fn has_image() -> bool {
        true
    }
}

impl FromFormData for NewGallery {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>> {
        Ok(Self {
            name: form_data.required_text_values["name"].clone(),
            featured_image_id: 0,
        })
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = galleries)]
pub struct UpdateGallery {
    pub name: Option<String>,
    pub featured_image_id: Option<i32>,
}

impl FormFields for UpdateGallery {
    fn get_optional_text_fields() -> Vec<&'static str> {
        vec!["name"]
    }
    fn has_image() -> bool {
        true
    }
    fn is_image_required() -> bool {
        false
    }
}

impl FromFormData for UpdateGallery {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>> {
        Ok(Self {
            name: form_data.optional_text_values.get("name").cloned().flatten(),
            featured_image_id: None,
        })
    }
}


#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(Gallery))]
#[diesel(belongs_to(Image))]
#[diesel(table_name =gallery_images)]
pub struct GalleryImage {
    pub id: i32,
    pub gallery_id: i32,
    pub image_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = gallery_images)]
pub struct NewGalleryImage {
    pub gallery_id: i32,
    pub image_id: i32,
}