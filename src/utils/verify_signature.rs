use crate::types::{U256, U512, Signature, SignatureVerification, EllipticCurve, Errors, Points, Point, ECpoint};
use crate::utils::hash_message;
use crate::group_math::m_inverse_mod;
use crate::ec_math::{point_add, scalar_mult};

pub fn verify_signature(public_key: ECpoint, message: &[u8], signature: &Signature, curve: & EllipticCurve) -> Result<SignatureVerification, Errors> {
    let public_key = public_key.to_finite_point();
    let z = hash_message(message, curve);
    #[allow(non_snake_case)]
    let G = Points::FinitePoint(Point::from(curve.g));
    let Signature{r, s} = *signature;
    let s_inv = m_inverse_mod(s, curve.n)?;
    let u1 = U256::from(s_inv.full_mul(z) % U512::from(curve.n));
    let u2 = U256::from(s_inv.full_mul(r) % U512::from(curve.n));
    #[allow(non_snake_case)]
    let u1G = scalar_mult(u1, &G, curve)?;
    let u2public_key = scalar_mult(u2, &Points::FinitePoint(public_key), curve)?;
    #[allow(non_snake_case)]
    let u1G = u1G.to_finite_point();
    let u2public_key = u2public_key.to_finite_point();
    let res = point_add(&Points::FinitePoint(u1G), &Points::FinitePoint(u2public_key), curve)?.to_finite_point();
    let Point{x, y: _} = res;

    match r % curve.n == x % curve.n {
        true => Ok(SignatureVerification::Successful),
        false => Ok(SignatureVerification::Failed)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{sign_message, make_keypair};

    #[test]
    fn test_verify_signature() -> Result<(), Errors> {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let mut rng = rand::thread_rng();
        #[allow(non_snake_case)]
        let (a, A) = make_keypair(&secp256k1)?;
        let msg = b"This is a test";
        let sig = sign_message(&mut rng, a, msg, &secp256k1)?;
        let ver = verify_signature(A, msg, &sig, &secp256k1)?;
        assert_eq!(ver, SignatureVerification::Successful);
        let msg = b"This should fail";
        let ver = verify_signature(A, msg, &sig, &secp256k1)?;
        assert_eq!(ver, SignatureVerification::Failed);
        #[allow(non_snake_case)]
        let (b, _) = make_keypair(&secp256k1)?;
        let msg = b"This is a test";
        let other_sig = sign_message(&mut rng, b, msg, &secp256k1)?;
        let ver = verify_signature(A, msg, &other_sig, &secp256k1)?;
        assert_eq!(ver, SignatureVerification::Failed);
        Ok(())
    }
}
