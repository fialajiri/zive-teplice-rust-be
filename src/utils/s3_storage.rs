use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::{Client as S3Client, Config};
use std::path::Path;

pub struct S3Storage {
    client: S3Client,
    bucket: String,
    region: String,
}

#[derive(Debug)]
pub struct UploadedImage {
    pub url: String,
    pub key: String,
}

#[derive(Debug)]
pub struct UploadResult {
    pub successful: Vec<UploadedImage>,
    pub failed: Vec<(String, String)>, // (filename, error message)
}

impl S3Storage {
    pub async fn new() -> Result<Self, String> {
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

        let client = S3Client::from_conf(shared_config);

        Ok(Self {
            client,
            bucket,
            region,
        })
    }

    pub async fn upload_image(&self, raw_data: &[u8], filename: &str) -> Result<UploadedImage, String> {
        println!("Uploading image to S3: {}", filename);

        let byte_stream = ByteStream::from(raw_data.to_vec());
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("jpg");
            
            let key = format!("uploads/{}.{}", uuid::Uuid::new_v4(), extension);

        match self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(byte_stream)
            .content_type(format!("image/{}", extension))
            .acl(ObjectCannedAcl::PublicRead)
            .send()
            .await
        {
            Ok(_) => {
                let url = format!(
                    "https://{}.s3.{}.amazonaws.com/{}",
                    self.bucket, self.region, key
                );
                Ok(UploadedImage { url, key })
            }
            Err(e) => Err(e.to_string()),
        }
    }

    

pub async fn upload_multiple_images(
        &self,
        images: Vec<(&[u8], String)>,
    ) -> UploadResult {
        let mut handles: Vec<tokio::task::JoinHandle<Result<(String, Result<UploadedImage, String>), tokio::task::JoinError>>> = Vec::with_capacity(images.len());
        
        // Start all uploads concurrently
        for (data, filename) in images {
            let data = data.to_vec(); // Clone the data for each task
            let filename = filename.clone();
            handles.push(tokio::spawn({
                let self_clone = self.clone();
                async move {
                    match self_clone.upload_image(&data, &filename).await {
                        Ok(uploaded) => Ok((filename, Ok(uploaded))),
                        Err(e) => Ok((filename, Err(e))),
                    }
                }
            }));
        }

        let mut successful = Vec::new();
        let mut failed = Vec::new();

        // Collect results
        for handle in handles {
            match handle.await {
                Ok(Ok((filename, result))) => {
                    match result {
                        Ok(uploaded) => successful.push(uploaded),
                        Err(error) => failed.push((filename, error)),
                    }
                },
                Ok(Err(e)) => failed.push(("unknown".to_string(), e.to_string())),
                Err(e) => failed.push(("unknown".to_string(), format!("Task failed: {}", e))),
            }
        }

        UploadResult {
            successful,
            failed,
        }
    }

    pub async fn delete_image(&self, key: &str) -> Result<(), String> {
        match self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete image: {}", e)),
        }
    }

    pub async fn delete_multiple_images(&self, keys: Vec<String>) -> Result<(), String> {
        let mut handles = Vec::with_capacity(keys.len());
        
        for key in keys {
            handles.push(tokio::spawn({
                let self_clone = self.clone();
                async move {
                    self_clone.delete_image(&key).await
                }
            }));
        }

        for handle in handles {
            match handle.await {
                Ok(result) => result?,
                Err(e) => return Err(format!("Task failed: {}", e)),
            }
        }

        Ok(())
    }
}

// Implement Clone for S3Storage
impl Clone for S3Storage {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            bucket: self.bucket.clone(),
            region: self.region.clone(),
        }
    }
}