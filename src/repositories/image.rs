pub struct ImageRepository;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::image::{Image, NewImage};
use crate::schema::images;

use aws_sdk_s3::{Client as S3Client, Config};

impl ImageRepository {
    pub async fn save_image(c: &mut AsyncPgConnection, image_field: Vec<u8>) -> QueryResult<Image> {
        let (width, height) = Self::get_image_dimensions(&image_field)
            .await
            .expect("Failed to get image dimensions");

        let (image_url, image_key) = Self::upload_to_s3(&image_field)
            .await
            .expect("Failed to upload image to s3");

        let new_image = NewImage {
            image_url,
            image_key,
            width: width as i32,
            height: height as i32,
        };

        diesel::insert_into(images::table)
            .values(new_image)
            .get_result(c)
            .await
    }

    pub async fn delete_image(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        //  Delete the image from S3

        diesel::delete(images::table.find(id)).execute(c).await
    }

    async fn get_image_dimensions(raw_data: &[u8]) -> Result<(u32, u32), String> {
        let img =
            image::load_from_memory(raw_data).map_err(|e| format!("Cannot decode image: {}", e))?;
        Ok((img.width(), img.height()))
    }

    async fn upload_to_s3(raw_data: &[u8]) -> Result<(String, String), String> {
        let bucket = std::env::var("AWS_BUCKET_NAME")
            .map_err(|e| format!("AWS_BUCKET_NAME not set: {}", e))?;
        let region = std::env::var("AWS_BUCKET_REGION")
            .map_err(|e| format!("AWS_BUCKET_REGION not set: {}", e))?;
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")
            .map_err(|e| format!("AWS_ACCESS_KEY_ID not set: {}", e))?;
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .map_err(|e| format!("AWS_SECRET_ACCESS_KEY not set: {}", e))?;

        let shared_config = Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(region.clone()))
            .credentials_provider(Credentials::new(
                access_key, secret_key, None, None, "example",
            ))
            .build();

        // Create S3 client
        let client = S3Client::from_conf(shared_config);

        println!("Uploading image to S3");       

        // Convert raw data to ByteStream
        let byte_stream = ByteStream::from(raw_data.to_vec());

        let key = format!("uploads/{}.{}", "test2", "jpg");

        // Upload to S3
        match client
            .put_object()
            .bucket(&bucket)
            .key(&key)
            .body(byte_stream)
            .content_type("image/jpg")
            .acl(ObjectCannedAcl::PublicRead)  
            .send()
            .await
        {
            Ok(_) => {
                // Construct the S3 URL
                let image_url = format!("https://{}.s3.{}.amazonaws.com/{}", bucket, region, key);
                Ok((image_url, key))
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
