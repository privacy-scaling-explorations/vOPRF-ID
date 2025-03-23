#![allow(dead_code)]

use std::fs;

use ark_ed_on_bn254::Fr;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;
use once_cell::sync::Lazy;
use rand::thread_rng;

const PRIVATE_KEY_FILE: &str = "./private_key.txt";

static PRIVATE_KEY: Lazy<Fr> = Lazy::new(|| {
    if let Ok(data) = fs::read(PRIVATE_KEY_FILE) {
        CanonicalDeserialize::deserialize_compressed(&mut data.as_slice()).unwrap()
    } else {
        let key = Fr::rand(&mut thread_rng());
        let mut serialized = Vec::new();
        key.serialize_compressed(&mut serialized).unwrap();
        fs::write(PRIVATE_KEY_FILE, serialized).unwrap();
        key
    }
});

fn main() {
    println!("Private key: {:?}", PRIVATE_KEY.0);
}
