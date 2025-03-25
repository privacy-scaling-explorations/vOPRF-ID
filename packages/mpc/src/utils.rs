#![allow(unused)]

use std::fs;

use ark_ec::models::twisted_edwards::Affine;
use ark_ed_on_bn254::{EdwardsConfig, EdwardsProjective, Fq, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{str::FromStr, UniformRand};
use num_bigint::BigUint;
use once_cell::sync::Lazy;
use rand::thread_rng;

const PRIVATE_KEY_FILE: &str = "./private_key.txt";

pub fn twisted_to_edwards(x: Fq, y: Fq) -> (Fq, Fq) {
    let sqrt_a: Fq = BigUint::from_str(
        "7214280148105020021932206872019688659210616427216992810330019057549499971851",
    )
    .unwrap()
    .into();
    let x_edwards = x * sqrt_a;
    let y_edwards = y;
    (x_edwards, y_edwards)
}

pub fn edwards_to_twisted(x: Fq, y: Fq) -> (Fq, Fq) {
    let sqrt_a_inv: Fq = BigUint::from_str(
        "2957874849018779266517920829765869116077630550401372566248359756137677864698",
    )
    .unwrap()
    .into();
    let x_twisted = x * sqrt_a_inv;
    let y_twisted = y;
    (x_twisted, y_twisted)
}

pub static KEYS: Lazy<(Fr, EdwardsProjective)> = Lazy::new(|| {
    let x: Fq = BigUint::from_str(
        "5299619240641551281634865583518297030282874472190772894086521144482721001553",
    )
    .expect("Invalid x-coordinate")
    .into();
    let y: Fq = BigUint::from_str(
        "16950150798460657717958625567821834550301663161624707787222815936182638968203",
    )
    .expect("Invalid y-coordinate")
    .into();
    let (x, y) = twisted_to_edwards(x, y);
    let base_point: EdwardsProjective = Affine::<EdwardsConfig>::new(x, y).into();

    if let Ok(data) = fs::read(PRIVATE_KEY_FILE) {
        let private_key =
            CanonicalDeserialize::deserialize_compressed(&mut data.as_slice()).unwrap();
        let public_key = base_point * private_key;
        (private_key, public_key)
    } else {
        let private_key = Fr::rand(&mut thread_rng());
        let public_key = base_point * private_key;
        let mut serialized = Vec::new();
        private_key.serialize_compressed(&mut serialized).unwrap();
        fs::write(PRIVATE_KEY_FILE, serialized).unwrap();
        (private_key, public_key)
    }
});
