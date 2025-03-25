#![allow(unused)]
#![allow(clippy::let_unit_value)]

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateRequest {
    pub commitment1: String,
    pub commitment2: Commitment2,
    pub proof: Proof,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commitment2 {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    // ZK proof fields will be added based on the circuit implementation
}

#[derive(Serialize)]
struct EvaluateResponse {
    // result: EdwardsProjective,
    // dleq_proof: DleqProof,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Invalid point")]
    InvalidPoint,
    #[error("Invalid proof")]
    InvalidProof,
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::InvalidPoint => StatusCode::BAD_REQUEST,
            Error::InvalidProof => StatusCode::UNAUTHORIZED,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": {
                "code": format!("{:?}", self),
                "message": self.to_string()
            }
        }))
    }
}

async fn evaluate_handler(req: web::Json<EvaluateRequest>) -> Result<HttpResponse, Error> {
    let result = todo!();
    let dleq_proof = todo!();
    Ok(HttpResponse::Ok().json(EvaluateResponse {}))
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/v1/evaluate", web::post().to(evaluate_handler)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
