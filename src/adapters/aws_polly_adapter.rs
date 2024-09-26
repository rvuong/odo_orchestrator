use crate::adapters::aws_config::AwsConfig;
use crate::domain::ports::text_to_speech_service::TextToSpeechService;
use crate::web::Bytes;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_polly::types::{OutputFormat, VoiceId};
use aws_sdk_polly::Client as PollyClient;

pub struct AwsPollyAdapter {
    client: PollyClient,
}

impl AwsPollyAdapter {
    pub async fn new(aws_config: AwsConfig) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else(aws_config.region());
        let config = aws_config::from_env().region(region_provider).load().await;

        // Create a Polly client
        let polly_client = PollyClient::new(&config);

        Self {
            client: polly_client,
        }
    }
}

#[async_trait]
impl TextToSpeechService for AwsPollyAdapter {
    async fn synthesize(&self, text: &str, lang: &str) -> Result<Bytes, String> {
        let voice_id = match lang {
            "fr" => VoiceId::Lea,
            _ => VoiceId::Brian,
        };

        // Get audio from text
        let response = self
            .client
            .synthesize_speech()
            .text(text)
            .voice_id(voice_id)
            .output_format(OutputFormat::Mp3)
            .send()
            .await;

        // Gérer la réponse
        match response {
            Ok(output) => {
                let audio_stream = output
                    .audio_stream
                    .collect()
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(audio_stream.into_bytes())
            }
            Err(e) => Err(format!("AWS Polly Error: {}", e)),
        }
    }
}
