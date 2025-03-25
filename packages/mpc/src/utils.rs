#![allow(unused)]

use std::fs;

use k256::{
    elliptic_curve::{rand_core::OsRng, PrimeField},
    FieldBytes, ProjectivePoint, Scalar,
};
use once_cell::sync::Lazy;

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
