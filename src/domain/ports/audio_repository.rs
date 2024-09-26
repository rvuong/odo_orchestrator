use crate::web::Bytes;
use async_trait::async_trait;

#[async_trait]
pub trait AudioRepository {
    async fn save(&self, text: &str, audio: Bytes) -> Result<String, String>;

    fn get_filename(&self, text: String) -> String;
}
