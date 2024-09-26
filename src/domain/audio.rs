use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {
    text: String,
    lang: String,
    url: String,
    qrcode_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AudioRequest {
    text: String,
    lang: String,
}

impl AudioRequest {
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn lang(&self) -> String {
        self.lang.clone()
    }
}

impl Audio {
    pub fn new(
        text: String,
        lang: String,
        url: String,
        qrcode_url: Option<String>,
    ) -> Result<Self, String> {
        if text.is_empty() {
            return Err("Any Audio must have a text.".to_string());
        }

        Ok(Self {
            text,
            lang,
            url,
            qrcode_url,
        })
    }
}
