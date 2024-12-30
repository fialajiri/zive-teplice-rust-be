use chrono::NaiveDateTime;
use serde::Deserialize;
use diesel::prelude::*;
use serde::Serialize;

use crate::models::image::Image;
use crate::schema::{galleries, gallery_images};

#[derive(Queryable, Serialize, Deserialize, Debug)]
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