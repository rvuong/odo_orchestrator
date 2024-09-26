use async_trait::async_trait;

#[async_trait]
pub trait QrCodeService {
    async fn create(&self, input_url: &str) -> Result<String, String>;
}
