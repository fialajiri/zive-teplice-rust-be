use rocket::http::ContentType;
use rocket::{response::status::Custom, Data};
use rocket_multipart_form_data::mime::Mime;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField,
};
use serde_json::Value;

use crate::errors::bad_request_error;

const MAX_IMAGE_SIZE: u64 = 30 * 1024 * 1024; // 30 MB

#[derive(Debug)]
pub struct ImageFormData {
    pub raw_data: Vec<u8>,
    pub file_name: Option<String>,
    pub content_type: Option<Mime>,
}

pub struct ProgramFormConfig<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ProgramFormConfig<'a> {
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get_options(&self) -> MultipartFormDataOptions<'a> {
        MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::raw("image")
                .size_limit(MAX_IMAGE_SIZE)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
            MultipartFormDataField::text("title"),
            MultipartFormDataField::text("text"),
            MultipartFormDataField::text("event_id"),
        ])
    }
}

// Helper struct to parse and validate form data
#[derive(Debug)]
pub struct ProgramFormData {
    pub title: String,
    pub text: String,
    pub event_id: i32,
    pub image_field: ImageFormData,
}

impl ProgramFormData {
    pub async fn from_multipart(
        content_type: &ContentType,
        data: Data<'_>,
        config: &ProgramFormConfig<'_>,
    ) -> Result<Self, Custom<Value>> {
        let form = MultipartFormData::parse(content_type, data, config.get_options())
            .await
            .map_err(|e| bad_request_error(e.into()))?;

        let title = Self::get_text_field(&form, "title")?;
        let text = Self::get_text_field(&form, "text")?;
        let event_id = Self::get_event_id(&form)?;
        let image_field = Self::get_image_field(&form)?;

        Ok(Self {
            title,
            text,
            event_id,
            image_field,
        })
    }

    fn get_text_field(
        form: &MultipartFormData,
        field_name: &'static str,
    ) -> Result<String, Custom<Value>> {
        form.texts
            .get(field_name)
            .and_then(|fields| fields.first())
            .map(|field| field.text.clone())
            .ok_or(bad_request_error(format!("Missing: {}", field_name).into()))
    }

    fn get_event_id(form: &MultipartFormData) -> Result<i32, Custom<Value>> {
        Self::get_text_field(form, "event_id")?
            .parse()
            .map_err(|e| bad_request_error(format!("Invalid event_id: {}", e).into()))
    }

    fn get_image_field(form: &MultipartFormData) -> Result<ImageFormData, Custom<Value>> {
        let field = form
            .raw
            .get("image")
            .and_then(|fields| fields.first())
            .ok_or_else(|| bad_request_error("image".into()))?;
    
        Self::get_image_form_data(field)
    }
    
    // Helper to transform a `RawField` into your `ImageFormData`
    fn get_image_form_data(field: &RawField) -> Result<ImageFormData, Custom<Value>> {
        Ok(ImageFormData {
            raw_data: field.raw.clone(),
            file_name: field.file_name.clone(),
            content_type: field.content_type.clone(),
        })
    }
}
