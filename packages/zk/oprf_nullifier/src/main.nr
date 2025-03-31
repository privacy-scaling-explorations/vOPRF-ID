use bignum::{BigNum, bignum::BigNumTrait};
use noir_bigcurve::curves::secp256k1::Secp256k1;
use noir_bigcurve::curves::secp256k1::Secp256k1_Fq_Params;
use std::hash::sha256;

fn main() {
    let x: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from_be_bytes::<32>([
        0x9d, 0x30, 0x46, 0x64, 0xa1, 0xba, 0xb1, 0x7a, 0x65, 0x09, 0x86, 0x63, 0x60, 0x6f, 0x82,
        0x26, 0x11, 0x1c, 0x82, 0xdb, 0x56, 0x65, 0x31, 0xcd, 0x2e, 0xa1, 0x1c, 0xaf, 0x96, 0xdb,
        0x68, 0xe9,
    ]);
    let y: BigNum<3, 256, Secp256k1_Fq_Params> = BigNum::from_be_bytes::<32>([
        0xae, 0xba, 0xfc, 0x6c, 0x01, 0xc0, 0xf0, 0xeb, 0x49, 0xdc, 0x5b, 0x79, 0xa7, 0x79, 0x6a,
        0xa3, 0x89, 0x72, 0x35, 0xda, 0x43, 0x6f, 0x50, 0xf3, 0x93, 0x80, 0x85, 0xbd, 0xe9, 0x38,
        0x36, 0x0b,
    ]);
    let point: Secp256k1 = Secp256k1 { x, y, is_infinity: false };
    let hash = sha256_point(point);
    println(hash);
    print(sha256([
        226, 37, 142, 19, 52, 141, 191, 25, 1, 198, 62, 19, 29, 187, 37, 11, 38, 108, 18, 175, 75,
        240, 200, 174, 211, 133, 182, 126, 217, 168, 150, 85,
    ]))
}

fn sha256_point(point: Secp256k1) -> [u8; 32] {
    let x_bytes = point.x.to_le_bytes::<32>();
    let y_bytes = point.y.to_le_bytes::<32>();

    let mut data: [u8; 65] = [0; 65];
    data[0] = 0x04;

    for i in 0..32 {
        data[i + 1] = x_bytes[i];
    }
    for i in 0..32 {
        data[i + 33] = y_bytes[i];
    }
    sha256(data)
}

#[test]
fn test_main() {
    main();

    // Uncomment to make test fail
    // main(1, 1);
}
