#![allow(unused)]
#![allow(clippy::let_unit_value)]

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, ResponseError};
use k256::{
    elliptic_curve::{
        group::GroupEncoding,
        sec1::{FromEncodedPoint, ToEncodedPoint},
        subtle::CtOption,
    },
    AffinePoint, EncodedPoint, ProjectivePoint,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::{
    ecpoint_to_projective, projective_to_ecpoint, verify_zk_proof, DleqProof, ECPoint, Proof, KEYS,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateRequest {
    pub commitment1: String,
    pub commitment2: ECPoint,
    pub proof: Proof,
}

#[derive(Serialize)]
struct EvaluateResponse {
    result: ECPoint,
    dleq_proof: DleqProof,
}

#[derive(Error, Debug)]
pub enum Error {
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
    verify_zk_proof(&req.proof)?;

    // Convert ECPoint to ProjectivePoint
    let commitment2_point = ecpoint_to_projective(&req.commitment2)?;

    // Perform scalar multiplication with private key
    let result_point = commitment2_point * KEYS.0;

    // Convert result to ECPoint
    let result = projective_to_ecpoint(&result_point);

    // Generate DLEQ proof that shows the same private key was used
    let dleq_proof = DleqProof::new(&commitment2_point);

    Ok(HttpResponse::Ok().json(EvaluateResponse { result, dleq_proof }))
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/v1/evaluate", web::post().to(evaluate_handler)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
