use rocket_multipart_form_data::mime::Mime;
use rocket_multipart_form_data::{mime, MultipartFormDataField, MultipartFormDataOptions};

const MAX_IMAGE_SIZE: u64 = 30 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ImageFormData {
    pub raw_data: Vec<u8>,
    pub file_name: Option<String>,
    pub content_type: Option<Mime>,
}

pub trait FormFields {
    fn get_required_text_fields() -> Vec<&'static str> {
        vec![]
    }
    fn get_optional_text_fields() -> Vec<&'static str> {
        vec![]
    }
    fn get_required_number_fields() -> Vec<&'static str> {
        vec![]
    }
    fn get_optional_number_fields() -> Vec<&'static str> {
        vec![]
    }
    fn has_image() -> bool {
        true
    }
    fn is_image_required() -> bool {
        true
    }
}

pub struct FormConfig<'a, T: FormFields> {
    phantom: std::marker::PhantomData<(&'a (), T)>,
}

impl<'a, T: FormFields> FormConfig<'a, T> {
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get_options(&self) -> MultipartFormDataOptions<'a> {
        let mut fields = Vec::new();

        if T::has_image() {
            fields.push(
                MultipartFormDataField::raw("image")
                    .size_limit(MAX_IMAGE_SIZE)
                    .content_type_by_string(Some(mime::IMAGE_STAR))
                    .unwrap(),
            );
        }

        for field_name in T::get_required_text_fields() {
            fields.push(MultipartFormDataField::text(field_name));
        }

        for field_name in T::get_optional_text_fields() {
            fields.push(MultipartFormDataField::text(field_name));
        }

        for field_name in T::get_required_number_fields() {
            fields.push(MultipartFormDataField::text(field_name));
        }

        for field_name in T::get_optional_number_fields() {
            fields.push(MultipartFormDataField::text(field_name));
        }

        MultipartFormDataOptions::with_multipart_form_data_fields(fields)
    }
}
