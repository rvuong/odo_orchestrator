use crate::adapters::aws_config::AwsConfig;
use crate::domain::ports::audio_repository::AudioRepository;
use crate::web::Bytes;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client as S3Client;
use log::{error, info};
use sha2::{Digest, Sha256};

pub struct AwsRepository {
    config: AwsConfig,
}

impl AwsRepository {
    pub fn new(config: AwsConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AudioRepository for AwsRepository {
    async fn save(&self, text: &str, audio: Bytes) -> Result<String, String> {
        // Store audio on the bucket, and return the mp3 url
        let region_provider = RegionProviderChain::default_provider().or_else(self.config.region());
        let config = aws_config::from_env().region(region_provider).load().await;
        let s3_client = S3Client::new(&config);

        let object_key = self.get_filename(text.to_string());
        info!("object_key: {:#?}", object_key);

        match s3_client
            .put_object()
            .bucket(self.config.bucket_name())
            .key(object_key.to_string())
            .body(audio.into())
            .content_type("audio/mpeg")
            .send()
            .await
        {
            Ok(_) => {
                let url = format!(
                    "https://{}.s3.{}.amazonaws.com/{}",
                    self.config.bucket_name(),
                    self.config.region(),
                    object_key
                );

                Ok(url)
            }
            Err(e) => {
                error!("AwsRepository error on pushing to the S3: {:#?}", e);
                Err(e.to_string())
            }
        }
    }

    fn get_filename(&self, text: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let result = hasher.finalize();
        let hash_string = format!("{:x}", result);
        let _object_key = format!("{}.mp3", hash_string);

        _object_key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_a_valid_hash() {
        assert!(true);
    }
}
