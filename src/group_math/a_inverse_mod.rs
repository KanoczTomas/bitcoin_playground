use crate::types::{U256, Errors};

/// Returns the additive inverse of k modulo p.
/// This function returns the only integer x such that (x + k) % p == 0.
pub fn a_inverse_mod<T: Into<U256>, U: Into<U256>>(k: T, p: U) -> Result<U256, Errors> {
    let (k, p) = (k.into(), p.into());
    if k == U256::zero(){
        return Ok(k);
    }
    if p == U256::zero() {
        return Err(Errors::ZeroModulo);
    }
    //we deal with k > p by taking its reminder
    let k = k % p;
    let x = p - k;
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EllipticCurve;

    #[test]
    fn test_a_inverse_mod() {
        let p = EllipticCurve::secp256k1_factory().p;
        let k = U256::from_dec_str("51962848049517897314481377586705320001209492118704192225945377961561169702593").unwrap();
        let x = U256::from_dec_str("63829241187798298109089607421982587852060492546936371813512206046347664969070").unwrap();
        assert_eq!(a_inverse_mod(k, p), Ok(x));
        assert_eq!((k + x) % p, U256::zero());
        let p = U256::from(11);
        let k = U256::from(5);
        let x = U256::from(6);
        assert_eq!(a_inverse_mod(k, p), Ok(x));
        assert_eq!((k + x) % p, U256::zero());
        let p = U256::from(0);
        assert_eq!(a_inverse_mod(k, p), Err(Errors::ZeroModulo));
        let p = U256::from(10);
        let k = U256::from(5);
        let x = U256::from(6);
        assert_ne!(a_inverse_mod(k, p), Ok(x));
        assert_ne!((k + x) % p, U256::zero());
    }
}
