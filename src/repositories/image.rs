use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::image::{Image, NewImage};
use crate::schema::images;
use crate::utils::form_fields::ImageFormData;
use crate::utils::s3_storage::S3Storage;


pub struct ImageRepository {
    s3_storage: S3Storage,
}

impl ImageRepository {
    pub async fn new() -> Result<Self, String> {
        let s3_storage = S3Storage::new().await?;
        Ok(Self { s3_storage })
    }

    pub async fn save_image(
        &self,
        c: &mut AsyncPgConnection,
        image_field: ImageFormData,       
    ) -> QueryResult<Image> {
        let (width, height) = Self::get_image_dimensions(&image_field.raw_data)
            .await
            .expect("Failed to get image dimensions");

        let filename = image_field.file_name.unwrap_or_else(|| format!("image_{}.jpg", chrono::Utc::now().timestamp()));


        let uploaded = self
            .s3_storage
            .upload_image(&image_field.raw_data, &filename)
            .await
            .expect("Failed to upload image to s3");

        let new_image = NewImage {
            image_url: uploaded.url,
            image_key: uploaded.key,
            width: width as i32,
            height: height as i32,
        };

        diesel::insert_into(images::table)
            .values(new_image)
            .get_result(c)
            .await
    }

    pub async fn save_multiple_images(
        &self,
        images: &[ImageFormData],
    ) -> Result<Vec<Image>, String> {
        // Convert ImageFormData to the format expected by S3Storage
        let uploads: Vec<(&[u8], String)> = images
            .iter()
            .map(|image| {
                let filename = image.file_name
                    .clone()
                    .unwrap_or_else(|| format!("image_{}.jpg", chrono::Utc::now().timestamp()));
                (&image.raw_data[..], filename)
            })
            .collect();
    
        // Upload all images to S3 concurrently
        let upload_result = self.s3_storage.upload_multiple_images(uploads).await;
    
        println!("Successfully uploaded {} files", upload_result.successful.len());
        if !upload_result.failed.is_empty() {
            println!("Failed to upload {} files", upload_result.failed.len());
            for (filename, error) in &upload_result.failed {
                println!("Failed to upload {}: {}", filename, error);
            }
        }
    
        // Mock saving to database - just create Image structs
        let mock_images: Vec<Image> = upload_result.successful
            .into_iter()
            .map(|uploaded| Image {
                id: rand::random::<i32>(), // Mock ID
                image_url: uploaded.url,
                image_key: uploaded.key,
                width: 800,  // Mock dimensions
                height: 600,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            })
            .collect();
    
        Ok(mock_images)
    }

    pub async fn delete_image(&self, c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        // First get the image key from database
        let image = images::table.find(id).first::<Image>(c).await?;
        
        // Delete from S3
        self.s3_storage
            .delete_image(&image.image_key)
            .await
            .expect("Failed to delete image from S3");

        // Delete from database
        diesel::delete(images::table.find(id)).execute(c).await
    }

    async fn get_image_dimensions(raw_data: &[u8]) -> Result<(u32, u32), String> {
        let img =
            image::load_from_memory(raw_data).map_err(|e| format!("Cannot decode image: {}", e))?;
        Ok((img.width(), img.height()))
    }
}
