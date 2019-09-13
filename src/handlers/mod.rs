use super::AppState;
use actix_web::{error, Error, HttpRequest, HttpResponse, Json, Path, Responder};
use failure::Fail;
use crate::domain::models;
use crate::clients::insults;

#[derive(Fail, Debug)]
pub enum AnalyzerError {
    #[fail(display = "External Service Error")]
    ExternalServiceError,
    #[fail(display = "Activity Not Found Error")]
    ActivityNotFoundError,
}

impl error::ResponseError for AnalyzerError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AnalyzerError::ExternalServiceError => HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("external service error"),
            AnalyzerError::ActivityNotFoundError => HttpResponse::NotFound()
                .content_type("text/plain")
                .body("activity not found"),
        }
    }
}

pub fn health(_: &HttpRequest<AppState>) -> impl Responder {
    "OK".to_string()
}

pub fn json_error_handler(err: error::JsonPayloadError, _: &HttpRequest<AppState>) -> Error {
    error::InternalError::from_response(
        "",
        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"json error: {}"}}"#, err)),
    )
    .into()
}

pub fn get_insults(
    req: &HttpRequest<AppState>,
) -> Result<Json<models::InsultResponse>, AnalyzerError> {
    let log = &req.state().log;

    let insult_client = insults::new(reqwest::Client::new());

    insult_client.get_insult()
        .map_err(|e| {
            error!(log, "Get Insults ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}

pub fn get_insults_by_language(
    (req, language): (HttpRequest<AppState>, Path<String>),
) -> Result<Json<models::InsultResponse>, AnalyzerError> {
    let log = &req.state().log;

    let insult_client = insults::new(reqwest::Client::new());

    insult_client.get_insult_by_languange(&language)
        .map_err(|e| {
            error!(log, "Get Activity Error: {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}

pub fn create_insult(
    (req, insult): (HttpRequest<AppState>, Json<models::InsultRequest>),
) -> Result<Json<models::InsultResponse>, AnalyzerError> {
    let log = &req.state().log;
    info!(log, "creating insult {:?}", insult);

    let insult_client = insults::new(reqwest::Client::new());

    insult_client.create_insult(&insult)
        .map_err(|e| {
            error!(log, "Create Activity ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}