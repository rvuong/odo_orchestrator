use crate::domain::audio::Audio;
use crate::domain::ports::audio_repository::AudioRepository;
use crate::domain::ports::qr_code_service::QrCodeService;
use crate::domain::ports::text_to_speech_service::TextToSpeechService;
use log::{debug, error};

pub struct AudioService {
    pub tts_service: Box<dyn TextToSpeechService>,
    pub audio_repository: Box<dyn AudioRepository>,
    pub qr_code_service: Box<dyn QrCodeService>,
}

impl AudioService {
    pub async fn create_audio(
        &self,
        text: &str,
        lang: &str,
        is_qr_code_expected: bool,
    ) -> Result<Audio, String> {
        debug!(target: "odomate", "create_audio()...");

        let audio_bytes = self.tts_service.synthesize(text, lang).await?;
        let audio_url = self.audio_repository.save(text, audio_bytes).await?;

        if audio_url.is_empty() {
            error!("AudioService create_audio error.");

            return Err("AudioService create_audio error.".to_string());
        }

        let qr_code = if is_qr_code_expected {
            let code_url = self.qr_code_service.create(&audio_url).await?;

            Some(code_url)
        } else {
            None
        };

        Audio::new(text.to_string(), lang.to_string(), audio_url, qr_code)
    }
}
