use crate::types::{U256, EllipticCurve, Errors, Points, Point, ECpoint};
use crate::ec_math::scalar_mult;
use crate::utils::generate_random_u256;

///Generates a random prive-public key pair.
pub fn make_keypair(curve: &EllipticCurve) -> Result<(U256, ECpoint), Errors> {
    let mut private_key;
    loop {
        private_key = generate_random_u256();
        if private_key >= U256::one() && private_key < curve.n {
            break;
        }
    }
    let public_key = scalar_mult(private_key, &Points::FinitePoint(Point::from(curve.g)), curve)?;
    Ok((private_key, public_key))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_keypair() -> Result<(), Errors>{
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let (private_key, public_key) = make_keypair(&secp256k1)?;
        assert_eq!(public_key, scalar_mult(private_key, &Points::FinitePoint(Point::from(secp256k1.g)), &secp256k1)?);
        Ok(())
    }
}
