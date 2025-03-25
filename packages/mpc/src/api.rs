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

use crate::utils::{DleqProof, KEYS};

#[derive(Debug, Serialize, Deserialize)]
pub struct ECPoint {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    // ZK proof fields will be added based on the circuit implementation
}

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

fn multiply_point(point: &ECPoint, private_key: &k256::Scalar) -> Result<ECPoint, Error> {
    // Convert string coordinates to big integers
    let x = hex::decode(&point.x).map_err(|_| Error::InvalidPoint)?;
    let y = hex::decode(&point.y).map_err(|_| Error::InvalidPoint)?;

    println!("Input point x: {}, y: {}", point.x, point.y);
    println!("Decoded x length: {}, y length: {}", x.len(), y.len());

    // Combine coordinates into SEC1 encoded point
    let mut encoded = Vec::with_capacity(65);
    encoded.push(0x04); // Uncompressed point marker
    encoded.extend_from_slice(&x);
    encoded.extend_from_slice(&y);

    println!("Encoded point length: {}", encoded.len());

    // Convert to curve point and verify it's valid
    let encoded_point = EncodedPoint::from_bytes(&encoded).map_err(|e| {
        println!("Failed to create encoded point: {:?}", e);
        Error::InvalidPoint
    })?;
    let point = ProjectivePoint::from_encoded_point(&encoded_point)
        .into_option()
        .ok_or(Error::InvalidPoint)?;

    // Perform scalar multiplication with private key
    let result_point = point * private_key;

    // Convert result to affine coordinates and get encoded point
    let affine = AffinePoint::from(result_point);
    let encoded_result = affine.to_encoded_point(false); // false = uncompressed

    // Extract x and y coordinates
    let result_x = encoded_result.x().unwrap();
    let result_y = encoded_result.y().unwrap();

    // Format result as hex strings
    Ok(ECPoint {
        x: hex::encode(result_x),
        y: hex::encode(result_y),
    })
}

async fn evaluate_handler(req: web::Json<EvaluateRequest>) -> Result<HttpResponse, Error> {
    let result = multiply_point(&req.commitment2, &KEYS.0)?;
    let dleq_proof = DleqProof::new();
    Ok(HttpResponse::Ok().json(EvaluateResponse { result, dleq_proof }))
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/v1/evaluate", web::post().to(evaluate_handler)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ProjectivePoint;

    #[test]
    fn test_point_multiplication() {
        // Create a test point (using generator point)
        let generator = ProjectivePoint::GENERATOR;

        // Convert generator to affine coordinates and extract x/y
        let affine = AffinePoint::from(generator);

        // Create ECPoint from the coordinates
        // Use EncodedPoint as an intermediate to get x and y coordinates
        let encoded_point = affine.to_encoded_point(false); // false = uncompressed
        let x_bytes = encoded_point.x().unwrap();
        let y_bytes = encoded_point.y().unwrap();

        let test_point = ECPoint {
            x: hex::encode(x_bytes),
            y: hex::encode(y_bytes),
        };

        println!("Test point x: {}", test_point.x);
        println!("Test point y: {}", test_point.y);

        // Test multiplication with private key
        let result = multiply_point(&test_point, &KEYS.0).unwrap();

        // Result should be the public key
        let public_key_affine = AffinePoint::from(KEYS.1);
        let encoded_pubkey = public_key_affine.to_encoded_point(false);
        let pubkey_x = encoded_pubkey.x().unwrap();
        let pubkey_y = encoded_pubkey.y().unwrap();

        let expected = ECPoint {
            x: hex::encode(pubkey_x),
            y: hex::encode(pubkey_y),
        };

        println!("Expected x: {}", expected.x);
        println!("Expected y: {}", expected.y);

        assert_eq!(result.x, expected.x);
        assert_eq!(result.y, expected.y);
    }
}
