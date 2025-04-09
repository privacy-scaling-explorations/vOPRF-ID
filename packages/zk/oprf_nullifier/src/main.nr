use bignum::{BigNum, bignum::BigNumTrait};
use noir_bigcurve::BigCurveTrait;
use noir_bigcurve::curves::secp256k1::{Secp256k1, Secp256k1_Fq_Params, Secp256k1Scalar};
use noir_bigcurve::scalar_field::ScalarFieldTrait;
use std::{hash::{poseidon2, sha256}, ops::Div};

fn main(
    user_id: Field,
    salt: Field,
    r: BigNum<3, 256, Secp256k1_Fq_Params>,
    user_id_commitment: pub Field,
    oprf_response: Secp256k1,
    nullifier: pub Field,
    chaum_pedersen_proof: DleqProof,
    oprf_pubkey: pub Secp256k1,
) {
    let r_scalar = Secp256k1Scalar::from_bignum(r);
    // 1. Verify consistency with Auth Proof
    let hashed_user_data = poseidon2::Poseidon2::hash([user_id, salt], 2);
    assert(hashed_user_data == user_id_commitment);

    // 2. Calculate oprf_commitment
    let user_id_bytes: [u8; 32] = user_id.to_le_bytes();
    let base_point = Secp256k1::hash_to_curve(user_id_bytes);
    let oprf_commitment = Secp256k1::msm([base_point], [r_scalar]);

    // 3. Verify consistency with OPRF
    dleq_proof_verification(
        Secp256k1::one(),
        oprf_response,
        oprf_pubkey,
        oprf_response,
        chaum_pedersen_proof,
    );

    // 4. Calculate nullifier: Poseidon({r^(-1) * oprf_response}.x)
    let one: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from(1);
    let r_inverse: BigNum<3, 256, Secp256k1_Fq_Params> = one.div(r);
    let computed_nullifier =
        Secp256k1::msm([oprf_response], [Secp256k1Scalar::from_bignum(r_inverse)]);
    let x = Field::from_le_bytes::<32>(computed_nullifier.x.to_le_bytes::<32>());
    let computed_nullifier = poseidon2::Poseidon2::hash([x], 1);

    assert(computed_nullifier == nullifier);
}

fn sha256_6_points(points: [Secp256k1; 6]) -> [u8; 32] {
    let mut data: [u8; 65 * 6] = [0; 65 * 6];

    // Process each point
    for i in 0..6 {
        // Start position for this point's data
        let offset = i * 65;

        // Add 0x04 marker for uncompressed point
        data[offset] = 0x04;

        // Get x and y coordinates as bytes
        let x_bytes = points[i].x.to_le_bytes::<32>();
        let y_bytes = points[i].y.to_le_bytes::<32>();

        // Copy x coordinate bytes
        for j in 0..32 {
            data[offset + 1 + j] = x_bytes[31 - j];
        }

        // Copy y coordinate bytes
        for j in 0..32 {
            data[offset + 33 + j] = y_bytes[31 - j];
        }
    }

    // Hash the concatenated data
    sha256(data)
}

struct DleqProof {
    c: BigNum<3, 256, Secp256k1_Fq_Params>,
    s: BigNum<3, 256, Secp256k1_Fq_Params>,
}

fn dleq_proof_verification(
    g: Secp256k1,
    h: Secp256k1,
    y: Secp256k1,
    z: Secp256k1,
    proof: DleqProof,
) {
    let s_scalar = Secp256k1Scalar::from_bignum(proof.s);
    let c_scalar = Secp256k1Scalar::from_bignum(proof.c);
    let a_prime = Secp256k1::msm([g, y], [s_scalar, c_scalar]);
    let b_prime = Secp256k1::msm([h, z], [s_scalar, c_scalar]);

    let hash = sha256_6_points([g, h, y, z, a_prime, b_prime]);

    let hash_bignum: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from_be_bytes(hash);
    assert(hash_bignum == proof.c);
}

#[test]
fn test_dleq_proof_verification() {
    let g = Secp256k1 {
        x: BigNum::from_be_bytes::<32>([
            0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
            0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
            0x16, 0xf8, 0x17, 0x98,
        ]),
        y: BigNum::from_be_bytes::<32>([
            0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d, 0xa4, 0xfb, 0xfc, 0x0e, 0x11,
            0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19, 0x9c, 0x47, 0xd0, 0x8f,
            0xfb, 0x10, 0xd4, 0xb8,
        ]),
        is_infinity: false,
    };
    let h = Secp256k1 {
        x: BigNum::from_be_bytes::<32>([
            0x87, 0xdd, 0x0a, 0x2e, 0x88, 0x0b, 0x43, 0x91, 0x6d, 0x11, 0x51, 0x17, 0x97, 0xfc,
            0x96, 0x39, 0xfa, 0x44, 0xeb, 0xec, 0x2e, 0x36, 0xee, 0x7f, 0x71, 0x1d, 0x51, 0x17,
            0x45, 0x50, 0x28, 0x34,
        ]),
        y: BigNum::from_be_bytes::<32>([
            0x43, 0xf5, 0x8f, 0x22, 0x1b, 0x1c, 0x62, 0x78, 0x8c, 0x28, 0xbf, 0x8b, 0x11, 0xbb,
            0x27, 0x1f, 0xb1, 0xf4, 0x66, 0xd5, 0xe4, 0xee, 0x56, 0xd1, 0x64, 0x94, 0x14, 0xd1,
            0xca, 0x02, 0x7b, 0xea,
        ]),
        is_infinity: false,
    };
    let y = Secp256k1 {
        x: BigNum::from_be_bytes::<32>([
            0xd2, 0x47, 0xa8, 0x6c, 0x95, 0xbd, 0xfb, 0x73, 0x08, 0x89, 0x30, 0xd1, 0x27, 0x5e,
            0xec, 0x5e, 0x81, 0x9f, 0x4f, 0x54, 0x92, 0x86, 0x6f, 0xde, 0x6a, 0x32, 0xfc, 0xe9,
            0x1f, 0xce, 0x60, 0x6f,
        ]),
        y: BigNum::from_be_bytes::<32>([
            0xe0, 0x9f, 0xd5, 0x13, 0xa0, 0xbd, 0x5e, 0x50, 0xfc, 0xf3, 0xc0, 0x57, 0xde, 0x93,
            0x8c, 0x53, 0xe8, 0x7f, 0x33, 0x69, 0x46, 0x79, 0x2f, 0x8b, 0xfd, 0xe2, 0xba, 0x96,
            0x47, 0xff, 0x6b, 0xb1,
        ]),
        is_infinity: false,
    };
    let z = Secp256k1 {
        x: BigNum::from_be_bytes::<32>([
            0x9b, 0x61, 0xb4, 0xc7, 0xb5, 0x1d, 0x4a, 0x9b, 0xc9, 0xa3, 0xd9, 0x94, 0x19, 0x20,
            0x5a, 0x79, 0x03, 0x6e, 0xf2, 0xe8, 0x4a, 0xf6, 0x7b, 0x12, 0xa5, 0x4c, 0x45, 0x7d,
            0xa6, 0x1b, 0x40, 0x31,
        ]),
        y: BigNum::from_be_bytes::<32>([
            0xf2, 0xd8, 0xf1, 0xe3, 0x78, 0xc9, 0xdd, 0x88, 0x35, 0x4f, 0x95, 0x8d, 0x7b, 0xd4,
            0x82, 0xb8, 0x25, 0xbf, 0xa8, 0x92, 0x84, 0x6b, 0x63, 0x90, 0x46, 0x3c, 0xc1, 0x4c,
            0x9c, 0x1a, 0x9b, 0xc3,
        ]),
        is_infinity: false,
    };
    let c = [
        39, 239, 139, 102, 162, 58, 212, 15, 66, 214, 5, 12, 112, 151, 201, 200, 171, 208, 32, 220,
        134, 170, 225, 29, 232, 145, 128, 145, 49, 140, 118, 213,
    ];
    let c: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from_be_bytes(c);

    let s = [
        212, 126, 161, 189, 151, 214, 139, 31, 86, 128, 147, 191, 115, 38, 241, 211, 34, 103, 44,
        66, 114, 80, 158, 1, 47, 55, 107, 115, 217, 173, 142, 128,
    ];
    let s: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from_be_bytes(s);

    dleq_proof_verification(g, h, y, z, DleqProof { c, s });
}
