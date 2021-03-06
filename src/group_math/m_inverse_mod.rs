use crate::types::{U256, Errors, U512};
use crate::group_math::a_inverse_mod;

/// Returns the multiplicative inverse of k modulo p.
/// This function returns the only integer x such that (x * k) % p == 1.
/// k must be non-zero and p must be a prime.
pub fn m_inverse_mod<T: Into<U512>, U: Into<U512>>(k: T, p: U) -> Result<U256, Errors>{
    let (k, p) = (k.into(), p.into());
    if k == 0.into(){
        return Err(Errors::ZeroDivision);
    }
    if p == 0.into() {
        return Err(Errors::ZeroModulo)
    }
    // Extended Euclidean algorithm.
    let (mut s, mut old_s) = (U512::from(0), U512::from(1));
    let (mut r, mut old_r) = (p, k);
    while r != U512::from(0) {
        let quotient = old_r / r;
        let mut tmp = r;
        r = a_inverse_mod((quotient * r) % p, p)?.into();
        r = (old_r + r) % p;
        old_r = tmp;
        tmp = s;
        s = a_inverse_mod((quotient * s) % p, p)?.into();
        s = (old_s + s) % p;
        old_s = tmp;
    }
    let (gcd, x) = (old_r, old_s);
    if gcd != U512::from(1) {
        return Err(Errors::NoMultiplicativeInverse(k.into(),p.into()));
    }
    Ok(x.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EllipticCurve;
    #[test]
    fn test_m_inverse_mod() {
        // 5 has no multiplicative inverse mod 10
        let p = U256::from(10);
        let k = U256::from(5);
        assert_eq!(m_inverse_mod(k, p), Err(Errors::NoMultiplicativeInverse(k,p)));
        // 5 has 9 as multiplicative inverse mod 11
        let p = U256::from(11);
        let k = U256::from(5);
        let x = U256::from(9);
        assert_eq!(m_inverse_mod(k, p), Ok(x));
        assert_eq!((k * x) % p, U256::one());
        // finding the multiplicative inverse of 0 is not defined
        assert_eq!(m_inverse_mod(U256::zero(), p), Err(Errors::ZeroDivision));
        // additive inverse of k (x) is not the multiplicative inverse
        let p = EllipticCurve::secp256k1_factory().p;
        let k = U256::from_dec_str("51962848049517897314481377586705320001209492118704192225945377961561169702593").unwrap();
        let x = U256::from_dec_str("63829241187798298109089607421982587852060492546936371813512206046347664969070").unwrap();
        assert_ne!(m_inverse_mod(k, p), Ok(x));
        assert_ne!(k.full_mul(x) % U512::from(p), U512::one());
        // this should pass as x is the multiplicative inverse of k
        let x = U256::from_dec_str("15770621123931935841922866852148091009166141688620356011139719709837462056333").unwrap();
        assert_eq!(m_inverse_mod(k, p), Ok(x));
        assert_eq!(k.full_mul(x) % U512::from(p), U512::one())
    }
}
