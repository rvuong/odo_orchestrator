use crate::web::Bytes;
use async_trait::async_trait;

#[async_trait]
pub trait TextToSpeechService {
    async fn synthesize(&self, text: &str, lang: &str) -> Result<Bytes, String>;
}
