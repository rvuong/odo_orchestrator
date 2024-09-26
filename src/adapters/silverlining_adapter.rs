use crate::adapters::aws_config::AwsConfig;
use crate::domain::ports::qr_code_service::QrCodeService;
use async_trait::async_trait;
use log::{debug, error};
use serde_json::{json, Value};

pub struct SilverliningAdapter {
    url: String,
    api_key: String,
    client: reqwest::Client,
}

impl SilverliningAdapter {
    pub async fn new(aws_config: AwsConfig) -> Self {
        let client = reqwest::Client::new();

        Self {
            url: aws_config.silverlining_url(),
            api_key: aws_config.silverlining_api_key(),
            client,
        }
    }
}

#[async_trait]
impl QrCodeService for SilverliningAdapter {
    async fn create(&self, input_url: &str) -> Result<String, String> {
        // Créer les données JSON
        let body = json!({
            "content": input_url,
            "return_as_file": false
        });

        let response = match self
            .client
            .post(&self.url)
            .header("x-api-key", &self.api_key)
            .json(&body)
            .send()
            .await
        {
            Ok(result) => result,
            Err(error) => {
                return Err(format!("QR Code API call error: {}", error));
            }
        };
        // debug!("response: {:#?}", response);

        if !response.status().is_success() {
            return Err(format!("QR Code API call status: {}", response.status()));
        }

        let response_json: Value = match response.json().await {
            Ok(json) => json,
            Err(err) => {
                error!("Error reading the JSON response: {:?}", err);

                return Err(format!("Error reading the JSON response: {:?}", err));
            }
        };

        // Extraire l'attribut "qrcode_url"
        if let Some(code_url) = response_json.get("qrcode_url") {
            debug!("code_url: {:#?}", code_url);
            let clean_code_url = code_url.to_string();
            let clean_code_url = clean_code_url.trim_matches('\"');
            debug!("clean_code_url: {:#?}", clean_code_url);

            return Ok(clean_code_url.to_string());
        } else {
            return Err(
                "Error reading the QR Code attribute in the JSON response: {:?}".to_string(),
            );
        }
    }
}
