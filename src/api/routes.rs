use crate::adapters::aws_config::AwsConfig;
use crate::adapters::aws_polly_adapter::AwsPollyAdapter;
use crate::adapters::aws_repository::AwsRepository;
use crate::adapters::silverlining_adapter::SilverliningAdapter;
use crate::api::app_config::AppConfig;
use crate::domain::audio::AudioRequest;
use crate::domain::audio_service::AudioService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[derive(Debug)]
pub struct AppState {
    pub aws_config: AwsConfig,
    pub app_config: AppConfig,
}

pub async fn healthcheck_handler(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let is_request_valid = data.app_config.is_api_key_valid(&req);

    if is_request_valid {
        HttpResponse::Ok().body("Healthy")
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

fn is_qr_code_expected(req: HttpRequest) -> bool {
    match req
        .query_string()
        .split('&')
        .find(|s| s.starts_with("get_qr_code="))
    {
        Some(get_qr_code) => matches!(get_qr_code.split('=').nth(1).unwrap_or("false"), "true"),
        None => false,
    }
}

pub async fn create_audio_handler(
    data: web::Data<AppState>,
    audio_request: web::Json<AudioRequest>,
    req: HttpRequest,
) -> impl Responder {
    if !data.app_config.is_api_key_valid(&req) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    // Get audio stream from text (tts_service)
    // Save audio MP3 to S3 (audio_repository)
    // Get QR Code url from audio MP3 url (audio_service)
    let tts_service = AwsPollyAdapter::new(data.aws_config.clone()).await;
    let audio_repository = Box::new(AwsRepository::new(data.aws_config.clone()));
    let qr_code_service = SilverliningAdapter::new(data.aws_config.clone()).await;
    let audio_service = AudioService {
        tts_service: Box::new(tts_service),
        audio_repository,
        qr_code_service: Box::new(qr_code_service),
    };

    match audio_service
        .create_audio(
            &(audio_request.text()),
            &(audio_request.lang()),
            is_qr_code_expected(req),
        )
        .await
    {
        Ok(audio) => HttpResponse::Ok().json(audio),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/healthcheck").route(web::get().to(healthcheck_handler)))
        .service(web::resource("/audio").route(web::post().to(create_audio_handler)));
}
