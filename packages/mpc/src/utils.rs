#![allow(unused)]

use std::fs;
use std::process::Command;

use crate::api::Error;
use k256::{
    elliptic_curve::{
        rand_core::OsRng,
        sec1::{FromEncodedPoint, ToEncodedPoint},
        PrimeField,
    },
    AffinePoint, EncodedPoint, FieldBytes, ProjectivePoint, Scalar,
};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const PRIVATE_KEY_FILE: &str = "./private_key.txt";

pub static KEYS: Lazy<(Scalar, ProjectivePoint)> = Lazy::new(|| {
    if let Ok(data) = fs::read(PRIVATE_KEY_FILE) {
        let scalar_bytes = FieldBytes::from_slice(&data);
        let private_key = Scalar::from_repr_vartime(*scalar_bytes).expect("invalid scalar bytes");
        let public_key = ProjectivePoint::GENERATOR * private_key;

        (private_key, public_key)
    } else {
        let private_key = Scalar::generate_vartime(&mut OsRng);
        let public_key = ProjectivePoint::GENERATOR * private_key;

        let bytes = private_key.to_bytes();
        fs::write(PRIVATE_KEY_FILE, bytes.as_slice()).unwrap();

        (private_key, public_key)
    }
});

#[derive(Debug, Serialize, Deserialize)]
pub struct ECPoint {
    pub x: String,
    pub y: String,
}

pub fn verify_zk_proof(proof: &[u8]) -> Result<(), Error> {
    // Create a unique temporary file for the proof
    let temp_proof_path = format!("./packages/mpc/generated/temp_proof_{}", Uuid::new_v4());

    // Write proof string to the temporary file
    fs::write(&temp_proof_path, proof).map_err(|e| {
        eprintln!("Failed to write proof to temp file: {}", e);
        Error::InvalidProof
    })?;

    // Execute the verification command
    let output = Command::new("bb")
        .arg("verify")
        .arg("-k")
        .arg("./packages/mpc/generated/vk")
        .arg("-p")
        .arg(&temp_proof_path)
        .output()
        .map_err(|e| {
            // Clean up temp file before returning error
            let _ = fs::remove_file(&temp_proof_path);
            eprintln!("Failed to execute verification command: {}", e);
            Error::InvalidProof
        })?;

    // Clean up temporary file
    let _ = fs::remove_file(&temp_proof_path);

    // Check if verification succeeded
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("Proof verification failed: {}", error_msg);
        Err(Error::InvalidProof)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DleqProof {
    pub c: String, // Challenge
    pub s: String, // Response
}

impl DleqProof {
    // Create a DLEQ proof for (private_key, private_key * G) and (private_key, private_key * point)
    pub fn new(point: &ProjectivePoint) -> Self {
        // Get our private key and points
        let private_key = KEYS.0;
        let g = ProjectivePoint::GENERATOR; // First base point G
        let h = *point; // Second base point (input point)

        // Our public key Y = private_key * G
        let y = KEYS.1;
        // Z = private_key * H (the result we're returning in the API)
        let z = h * private_key;

        // Generate a random nonce for the proof
        let k = Scalar::generate_vartime(&mut OsRng);

        // Calculate k*G and k*H
        let a = g * k; // A = k*G
        let b = h * k; // B = k*H

        // Convert points to affine for serialization
        let g_affine = AffinePoint::from(g).to_encoded_point(false);
        let h_affine = AffinePoint::from(h).to_encoded_point(false);
        let y_affine = AffinePoint::from(y).to_encoded_point(false);
        let z_affine = AffinePoint::from(z).to_encoded_point(false);
        let a_affine = AffinePoint::from(a).to_encoded_point(false);
        let b_affine = AffinePoint::from(b).to_encoded_point(false);

        // Create a secure challenge using SHA-256 hash of all relevant values
        let mut hasher = Sha256::new();
        hasher.update(g_affine.as_bytes());
        hasher.update(h_affine.as_bytes());
        hasher.update(y_affine.as_bytes());
        hasher.update(z_affine.as_bytes());
        hasher.update(a_affine.as_bytes());
        hasher.update(b_affine.as_bytes());

        let hash_result = hasher.finalize();

        // Convert hash to scalar
        let mut scalar_bytes = [0u8; 32];
        scalar_bytes.copy_from_slice(&hash_result[..32]);

        // Create challenge scalar from hash
        let c_scalar = Scalar::from_repr_vartime(FieldBytes::from(scalar_bytes)).unwrap();

        // Calculate response s = k - c * private_key
        let s = k - c_scalar * private_key;

        Self {
            c: hex::encode(c_scalar.to_bytes()),
            s: hex::encode(s.to_bytes()),
        }
    }

    // Verify the DLEQ proof: checks that (g, y) and (h, z) share the same discrete log
    pub fn verify(
        &self,
        g: &ProjectivePoint,
        h: &ProjectivePoint,
        y: &ProjectivePoint,
        z: &ProjectivePoint,
    ) -> bool {
        // Parse the challenge and response from hex strings
        let c_bytes = match hex::decode(&self.c) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let s_bytes = match hex::decode(&self.s) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Convert bytes to field bytes
        if c_bytes.len() != 32 || s_bytes.len() != 32 {
            return false;
        }

        let mut c_arr = [0u8; 32];
        let mut s_arr = [0u8; 32];
        c_arr.copy_from_slice(&c_bytes);
        s_arr.copy_from_slice(&s_bytes);

        // Convert to scalars
        let c_field_bytes = FieldBytes::from(c_arr);
        let s_field_bytes = FieldBytes::from(s_arr);

        let c_scalar = match Scalar::from_repr_vartime(c_field_bytes) {
            Some(s) => s,
            None => return false,
        };

        let s_scalar = match Scalar::from_repr_vartime(s_field_bytes) {
            Some(s) => s,
            None => return false,
        };

        // Reconstruct A' = s*G + c*Y
        let a_prime = (*g * s_scalar) + (*y * c_scalar);

        // Reconstruct B' = s*H + c*Z
        let b_prime = (*h * s_scalar) + (*z * c_scalar);

        // Convert points to affine for serialization
        let g_affine = AffinePoint::from(*g).to_encoded_point(false);
        let h_affine = AffinePoint::from(*h).to_encoded_point(false);
        let y_affine = AffinePoint::from(*y).to_encoded_point(false);
        let z_affine = AffinePoint::from(*z).to_encoded_point(false);
        let a_prime_affine = AffinePoint::from(a_prime).to_encoded_point(false);
        let b_prime_affine = AffinePoint::from(b_prime).to_encoded_point(false);

        // Compute challenge c' = Hash(g, h, y, z, a', b')
        let mut hasher = Sha256::new();
        hasher.update(g_affine.as_bytes());
        hasher.update(h_affine.as_bytes());
        hasher.update(y_affine.as_bytes());
        hasher.update(z_affine.as_bytes());
        hasher.update(a_prime_affine.as_bytes());
        hasher.update(b_prime_affine.as_bytes());

        let hash_result = hasher.finalize();

        // Convert hash to scalar
        let mut scalar_bytes = [0u8; 32];
        scalar_bytes.copy_from_slice(&hash_result[..32]);

        // Create challenge scalar from hash
        let c_prime_scalar = match Scalar::from_repr_vartime(FieldBytes::from(scalar_bytes)) {
            Some(s) => s,
            None => return false,
        };

        // Verify that c == c'
        c_scalar == c_prime_scalar
    }
}

// Helper function to convert ECPoint to ProjectivePoint
pub fn ecpoint_to_projective(point: &ECPoint) -> Result<ProjectivePoint, Error> {
    let x = hex::decode(&point.x).map_err(|_| Error::InvalidPoint)?;
    let y = hex::decode(&point.y).map_err(|_| Error::InvalidPoint)?;

    // Combine coordinates into SEC1 encoded point
    let mut encoded = Vec::with_capacity(65);
    encoded.push(0x04); // Uncompressed point marker
    encoded.extend_from_slice(&x);
    encoded.extend_from_slice(&y);

    // Convert to curve point and verify it's valid
    let encoded_point = EncodedPoint::from_bytes(&encoded).map_err(|_| Error::InvalidPoint)?;
    ProjectivePoint::from_encoded_point(&encoded_point)
        .into_option()
        .ok_or(Error::InvalidPoint)
}

// Helper function to convert ProjectivePoint to ECPoint
pub fn projective_to_ecpoint(point: &ProjectivePoint) -> ECPoint {
    // Convert to affine coordinates
    let affine = AffinePoint::from(*point);
    let encoded_point = affine.to_encoded_point(false); // false = uncompressed

    // Extract x and y coordinates
    let x_bytes = encoded_point.x().unwrap();
    let y_bytes = encoded_point.y().unwrap();

    // Format as hex strings
    ECPoint {
        x: hex::encode(x_bytes),
        y: hex::encode(y_bytes),
    }
}

pub fn limbs_to_hex(limbs: &[String]) -> String {
    let limb0 = BigUint::from_str_radix(limbs[0].trim_start_matches("0x"), 16).unwrap();
    let limb1 = BigUint::from_str_radix(limbs[1].trim_start_matches("0x"), 16).unwrap();
    let limb2 = BigUint::from_str_radix(limbs[2].trim_start_matches("0x"), 16).unwrap();

    // Define 2^120 as a BigUint
    let two = BigUint::from_u32(2).unwrap();
    let shift_120 = two.pow(120);

    // Combine the limbs: limb0 + limb1 * 2^120 + limb2 * 2^240 as described in Noir-BigNum
    let result = limb0 + (limb1 * &shift_120.clone()) + (limb2 * shift_120.pow(2));

    // Convert to hexadecimal string, remove "0x" prefix, and ensure 64 digits (256 bits)
    let hex_result = format!("{:064x}", result);

    hex_result
}

pub fn parse_public_inputs(proof: &[u8]) -> Result<(String, ECPoint), Error> {
    // Extract user_id_commitment (bytes 0-31)
    let user_id_commitment = format!("0x{}", hex::encode(&proof[0..32]));

    // Extract oprf_commitment limbs (6 chunks of 32 bytes each)
    let mut x_limbs = Vec::new();
    let mut y_limbs = Vec::new();

    // x limbs: bytes 32-63, 64-95, 96-127 (with padding)
    for i in (32..128).step_by(32) {
        let limb = &proof[i..i + 32];
        let hex = hex::encode(limb);
        // Remove leading zeros, but keep at least one digit
        let trimmed = hex.trim_start_matches('0');
        let limb_str = if trimmed.is_empty() {
            "0".to_string()
        } else {
            format!("0x{}", trimmed)
        };
        x_limbs.push(limb_str);
    }

    // y limbs: bytes 128-159, 160-191, 192-223
    for i in (128..224).step_by(32) {
        let limb = &proof[i..i + 32];
        let hex = hex::encode(limb);
        let trimmed = hex.trim_start_matches('0');
        let limb_str = if trimmed.is_empty() {
            "0".to_string()
        } else {
            format!("0x{}", trimmed)
        };
        y_limbs.push(limb_str);
    }

    let x = limbs_to_hex(&x_limbs);
    let y = limbs_to_hex(&y_limbs);

    let ecpoint = ECPoint { x, y };

    Ok((user_id_commitment, ecpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dleq_proof_verification() {
        // Get the private key and generator point
        let private_key = KEYS.0;
        let g = ProjectivePoint::GENERATOR;

        // Calculate the public key Y = private_key * G
        let y = g * private_key;

        // Create a random point H
        let h = g * Scalar::generate_vartime(&mut OsRng);

        // Calculate Z = private_key * H
        let z = h * private_key;

        // Generate the DLEQ proof
        let proof = DleqProof::new(&h);

        // Verify the proof
        assert!(
            proof.verify(&g, &h, &y, &z),
            "DLEQ proof verification failed"
        );

        // Negative test: wrong points should fail verification
        let wrong_z = h * Scalar::generate_vartime(&mut OsRng);
        assert!(
            !proof.verify(&g, &h, &y, &wrong_z),
            "DLEQ verification should fail with wrong Z"
        );
    }

    // #[test]
    // fn test_public_inputs() {
    //     println!("{}", std::env::current_dir().unwrap().display());
    //     let proof = fs::read("../zk/oprf_commitment/target/proof").unwrap();
    //     let (user_id_commitment, point) = parse_public_inputs(&proof).unwrap();
    //     println!("user_id_commitment: {}", user_id_commitment);
    //     println!("point: {:?}", point);

    //     let projective_point = ecpoint_to_projective(&point).unwrap();
    //     let ecpoint = projective_to_ecpoint(&projective_point);
    //     println!("ecpoint: {:?}", ecpoint);
    // }
}
