use bignum::bignum::BigNum;
use noir_bigcurve::curves::secp256k1::{Secp256k1, Secp256k1_Fq_Params, Secp256k1Scalar};
use noir_bigcurve::scalar_field::ScalarField;
use std::{hash::poseidon2, ops::Add};

fn main(
    user_id: Field,
    salt: Field,
    random_scalar: BigNum<3, 256, Secp256k1_Fq_Params>,
    user_id_commitment: pub Field,
    oprf_commitment: pub Secp256k1,
) {
    // Verify the first commitment is a hash of the user_id and salt
    let hashed_user_data = poseidon2::Poseidon2::hash([user_id, salt], 2);
    assert(hashed_user_data == user_id_commitment);

    // Convert user_id to bytes for curve hashing
    let user_id_bytes: [u8; 32] = user_id.to_le_bytes();

    // Hash user_id to a curve point
    let base_point = Secp256k1::hash_to_curve(user_id_bytes);

    // Convert the random scalar to the appropriate type
    let scalar = Secp256k1Scalar::from_bignum(random_scalar);

    // Compute the OPRF value (scalar multiplication)
    let computed_oprf = Secp256k1::msm([base_point], [scalar]);

    // Verify the OPRF commitment
    assert(computed_oprf == oprf_commitment);
}

#[test]
fn test_valid_oprf_commitment() {
    // Test parameters
    let user_id = 12345;
    let salt = 67890;
    let random_scalar: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from(2);

    // // Calculate the expected user_id_commitment
    let user_id_commitment = poseidon2::Poseidon2::hash([user_id, salt], 2);

    // // Calculate the expected oprf_commitment
    let user_id_bytes: [u8; 32] = user_id.to_le_bytes();
    let base_point = Secp256k1::hash_to_curve(user_id_bytes);
    let scalar = Secp256k1Scalar::from_bignum(random_scalar);
    let oprf_commitment = Secp256k1::msm([base_point], [scalar]);

    println(random_scalar);
    println(oprf_commitment);

    // Test with valid parameters - should pass
    main(
        user_id,
        salt,
        random_scalar,
        user_id_commitment,
        oprf_commitment,
    );
}

#[test(should_fail)]
fn test_invalid_user_id_commitment() {
    // Test parameters
    let user_id = 12345;
    let salt = 67890;
    let random_scalar: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from(42);

    // Calculate the expected oprf_commitment
    let user_id_bytes: [u8; 32] = user_id.to_le_bytes();
    let base_point = Secp256k1::hash_to_curve(user_id_bytes);
    let scalar = Secp256k1Scalar::from_bignum(random_scalar);
    let oprf_commitment = Secp256k1::msm([base_point], [scalar]);

    // Create an invalid user_id_commitment (using wrong salt)
    let invalid_user_id_commitment = poseidon2::Poseidon2::hash([user_id, salt + 1], 2);

    // This should fail because the user_id_commitment is invalid
    main(
        user_id,
        salt,
        random_scalar,
        invalid_user_id_commitment,
        oprf_commitment,
    );
}

#[test(should_fail)]
fn test_invalid_oprf_commitment() {
    // Test parameters
    let user_id = 12345;
    let salt = 67890;
    let random_scalar: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from(42);

    // Calculate the expected user_id_commitment
    let user_id_commitment = poseidon2::Poseidon2::hash([user_id, salt], 2);

    // Calculate an invalid oprf_commitment (using wrong scalar)
    let user_id_bytes: [u8; 32] = user_id.to_le_bytes();
    let base_point = Secp256k1::hash_to_curve(user_id_bytes);
    let invalid_scalar = Secp256k1Scalar::from_bignum(random_scalar.add(BigNum::from(1)));
    let invalid_oprf_commitment = Secp256k1::msm([base_point], [invalid_scalar]);

    // This should fail because the oprf_commitment is invalid
    main(
        user_id,
        salt,
        random_scalar,
        user_id_commitment,
        invalid_oprf_commitment,
    );
}
