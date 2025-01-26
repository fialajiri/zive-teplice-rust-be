use rocket::http::ContentType;
use rocket::{response::status::Custom, Data};
use rocket_multipart_form_data::MultipartFormData;
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::bad_request_error;
use crate::utils::form_fields::*;

#[derive(Debug)]
pub struct FormData {
    pub required_text_values: HashMap<String, String>,
    pub optional_text_values: HashMap<String, Option<String>>,
    pub required_number_values: HashMap<String, i32>,
    pub optional_number_values: HashMap<String, Option<i32>>,
    pub image_field: Option<ImageFormData>,
}

impl FormData {
    pub async fn from_multipart<T: FormFields>(
        content_type: &ContentType,
        data: Data<'_>,
        config: &FormConfig<'_, T>,
    ) -> Result<Self, Custom<Value>> {
        let form = MultipartFormData::parse(content_type, data, config.get_options())
            .await
            .map_err(|e| bad_request_error(e.into()))?;

        let mut required_text_values = HashMap::new();
        let mut optional_text_values = HashMap::new();
        let mut required_number_values = HashMap::new();
        let mut optional_number_values = HashMap::new();

        for field_name in T::get_required_text_fields() {
            let value = Self::get_required_text_field(&form, field_name)?;
            required_text_values.insert(field_name.to_string(), value);
        }

        for field_name in T::get_optional_text_fields() {
            let value = Self::get_optional_text_field(&form, field_name)?;
            optional_text_values.insert(field_name.to_string(), value);
        }

        for field_name in T::get_required_number_fields() {
            let value = Self::get_required_number_field(&form, field_name)?;
            required_number_values.insert(field_name.to_string(), value);
        }

        for field_name in T::get_optional_number_fields() {
            let value = Self::get_optional_number_field(&form, field_name)?;
            optional_number_values.insert(field_name.to_string(), value);
        }

        let image_field = if T::has_image() {
            match Self::get_image_field(&form) {
                Ok(image) => Some(image),
                Err(e) if T::is_image_required() => return Err(e),
                Err(_) => None,
            }
        } else {
            None
        };

        Ok(Self {
            required_text_values,
            optional_text_values,
            required_number_values,
            optional_number_values,
            image_field,
        })
    }

    fn get_required_text_field(
        form: &MultipartFormData,
        field_name: &'static str,
    ) -> Result<String, Custom<Value>> {
        form.texts
            .get(field_name)
            .and_then(|fields| fields.first())
            .map(|field| field.text.clone())
            .ok_or_else(|| {
                bad_request_error(format!("Missing required field: {}", field_name).into())
            })
    }

    fn get_optional_text_field(
        form: &MultipartFormData,
        field_name: &'static str,
    ) -> Result<Option<String>, Custom<Value>> {
        Ok(form
            .texts
            .get(field_name)
            .and_then(|fields| fields.first())
            .map(|field| field.text.clone()))
    }

    fn get_required_number_field(
        form: &MultipartFormData,
        field_name: &'static str,
    ) -> Result<i32, Custom<Value>> {
        Self::get_required_text_field(form, field_name)?
            .parse()
            .map_err(|e| bad_request_error(format!("Invalid {}: {}", field_name, e).into()))
    }

    fn get_optional_number_field(
        form: &MultipartFormData,
        field_name: &'static str,
    ) -> Result<Option<i32>, Custom<Value>> {
        match form.texts.get(field_name) {
            Some(fields) if !fields.is_empty() => {
                let value = fields[0].text.clone();
                if value.is_empty() {
                    Ok(None)
                } else {
                    value.parse().map(Some).map_err(|e| {
                        bad_request_error(format!("Invalid {}: {}", field_name, e).into())
                    })
                }
            }
            _ => Ok(None),
        }
    }

    fn get_image_field(form: &MultipartFormData) -> Result<ImageFormData, Custom<Value>> {
        let field = form
            .raw
            .get("image")
            .and_then(|fields| fields.first())
            .ok_or_else(|| bad_request_error("Missing: image".into()))?;

        Ok(ImageFormData {
            raw_data: field.raw.clone(),
            file_name: field.file_name.clone(),
            content_type: field.content_type.clone(),
        })
    }
}

pub trait FromFormData: Sized {
    fn from_form_data(form_data: FormData) -> Result<Self, Custom<Value>>;
}
