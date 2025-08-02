// ==============================================
// deposit_address.rs — Domex Public Key Deriver
// ==============================================
// Derives public key (x, y) from a 32-byte secret key using Pasta (Pallas) curve.
// Used during ZK onboarding to populate public inputs with (pk_x, pk_y).

use pasta_curves::pallas::{Point, Scalar as Fr};
use pasta_curves::arithmetic::CurveAffine;
use pasta_curves::Fp;

/// Derives a public key (x, y) from a 32-byte secret key.
///
/// Returns:
/// - Tuple (pk_x, pk_y) as Pasta Fp field elements
/// - Used in Domex ZK onboarding proof as public identity fields
pub fn derive_public_key(sk_bytes: &[u8; 32]) -> (Fp, Fp) {
    // Convert secret key bytes to Pasta scalar (Fr)
    let sk_scalar = Fr::from_bytes(sk_bytes)
        .expect("Invalid secret key: not a valid Pasta scalar");

    // Elliptic curve multiplication to get public key point
    let public_point: Point = Point::generator() * sk_scalar;
    let affine = public_point.to_affine();

    // Extract x and y as Fp field elements (Pasta Fp)
    (affine.x, affine.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_derivation() {
        let sk = [1u8; 32]; // Sample secret key
        let (x, y) = derive_public_key(&sk);
        println!("Derived Public Key:\nx = {:?}\ny = {:?}", x, y);
    }
}
