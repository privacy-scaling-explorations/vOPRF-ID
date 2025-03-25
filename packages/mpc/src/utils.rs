#![allow(unused)]

use std::fs;

use k256::{
    elliptic_curve::{
        rand_core::OsRng,
        sec1::{FromEncodedPoint, ToEncodedPoint},
        PrimeField,
    },
    AffinePoint, FieldBytes, ProjectivePoint, Scalar,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
        let h = ProjectivePoint::GENERATOR * Scalar::generate_vartime(&mut OsRng);

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
}
