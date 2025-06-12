use aws_config::{SdkConfig, meta::region::RegionProviderChain};
use aws_sdk_s3::{Client, config::Credentials, primitives::ByteStream};
use axum::body::Bytes;
use uuid::Uuid;

use crate::{
    config::{self, config},
    services::error::{Result, ServiceError},
};

pub struct S3Service {
    pub client: Client,
    config: SdkConfig,
}

impl S3Service {
    pub async fn new() -> Self {
        let credentials = Credentials::new(
            config().AWS_ACCESS_KEY_ID.clone(),
            config().AWS_SECRET_ACCESS_KEY.clone(),
            None,
            None,
            "aws-creds",
        );

        let region = config().AWS_BUCKET_REGION.as_str();
        let region_provider = RegionProviderChain::default_provider().or_else(region);

        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(credentials)
            .load()
            .await;

        let client = Client::new(&config);
        Self { client, config }
    }

    pub async fn get_public_url(&self, key: &String) -> Result<String> {
        let url = format!(
            "https://{}.s3.amazonaws.com/${}",
            config::config().AWS_BUCKET_NAME,
            key
        );
        Ok(url)
    }

    pub async fn upload_file(
        &self,
        s3_folder: String,
        file: Bytes,
        filename: Option<String>,
    ) -> Result<String> {
        let kind = infer::get(&file);
        let file_data = match kind {
            Some(t) => Ok((t.mime_type(), t.extension())),
            None => Err(ServiceError::S3Error("Invalid file".to_string())),
        }?;

        let (mime_type, ext) = file_data;
        let key = match filename {
            Some(name) => format!("{}/{}.{}", s3_folder, name, ext),
            None => format!("{}{}.{}", s3_folder, Uuid::new_v4().to_string(), ext),
        };

        let builder = self
            .client
            .put_object()
            .bucket(config::config().AWS_BUCKET_NAME.clone())
            .body(ByteStream::from(file.clone()))
            .key(&key)
            .content_type(mime_type);

        builder.send().await?;
        Ok(key)
    }
}
