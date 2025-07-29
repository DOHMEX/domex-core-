// deposit_address.rs
// Derives public key (x, y) from private key using Pasta curve (Pallas)
// Used to generate native chain deposit address and ZK public inputs

use pasta_curves::pallas::{Point, Scalar as Fr};
use pasta_curves::arithmetic::CurveAffine;
use pasta_curves::group::GroupEncoding;
use pasta_curves::Fp;

/// Derives a public key (x, y) from a 32-byte secret key.
/// Returns affine coordinates as field elements (Fp).
pub fn derive_public_key(sk_bytes: &[u8; 32]) -> (Fp, Fp) {
    // Convert sk bytes to Pasta scalar (Fr)
    let sk_scalar = Fr::from_bytes(sk_bytes)
        .expect("Invalid secret key: not a valid Pasta scalar");

    // EC multiplication to get public point
    let public_point: Point = Point::generator() * sk_scalar;
    let affine = public_point.to_affine();

    // Extract (x, y) coordinates as Pasta Fp
    let x = affine.x;
    let y = affine.y;

    (x, y)
}
