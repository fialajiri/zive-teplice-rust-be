pub struct ImageRepository;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::image::{Image, NewImage};
use crate::schema::images;

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

    async fn upload_to_s3(_raw_data: &[u8]) -> Result<(String, String), String> {
        // In a real app, you’d do something like:
        //
        // let s3_client = ... // get from your rocket::State<aws_sdk_s3::Client>
        // let bucket = "your-bucket";
        // let unique_key = generate_key(...) // create a unique file key
        // s3_client.put_object()
        //     .bucket(bucket)
        //     .key(&unique_key)
        //     .body(ByteStream::from(raw_data.to_vec()))
        //     .send()
        //     .await
        //     .map_err(|e| format!("S3 upload failed: {}", e))?;
        //
        // let image_url = format!("https://{}.s3.amazonaws.com/{}", bucket, unique_key);
        // Ok((image_url, unique_key));

        // For this example, just pretend we succeeded:
        Ok((
            "https://fake-bucket.s3.amazonaws.com/fake_key.jpg".to_string(),
            "fake_key.jpg".to_string(),
        ))
    }
}
