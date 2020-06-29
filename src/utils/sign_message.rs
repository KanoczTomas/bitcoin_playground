use crate::types::{U512, U256, Signature, EllipticCurve, ECpoint, Points, Point, Errors};
use crate::utils::hash_message;
use crate::ec_math::scalar_mult;
use crate::group_math::m_inverse_mod;
use crate::traits::RandU256;

///Creates the hash of a message and sighns it with private_key
pub fn sign_message<R: RandU256>(rng: &mut R, private_key: U256, message: &[u8], curve: &EllipticCurve) -> Result<Signature, Errors> {
    // let mut rng = rand::thread_rng();
    let z = U512::from(hash_message(message, curve));
    #[allow(non_snake_case)]
    let G = Point::from(curve.g);
    let mut k;
    let (mut r, mut s) = (U256::zero(), U512::zero());
    while r == U256::zero() || s == U512::zero() {
        k = rng.gen_u256_range(&U256::one(), &curve.n);
        match scalar_mult(k, &Points::FinitePoint(G), curve)? {
            ECpoint::Infinity => continue,
            ECpoint::OnCurve(p) => {
                let Point {x, y: _} = p;
                r = x % curve.n;
                //s = k_inverse * (z + r*private_key) (mod n)
                let k_inverse = U512::from(m_inverse_mod(k, curve.n)?);
                let r_times_private_key = r.full_mul(private_key) % U512::from(curve.n);
                let z_plus_r_times_private_key = (z + r_times_private_key) % U512::from(curve.n);
                s = (k_inverse * z_plus_r_times_private_key)  % U512::from(curve.n);
            }
        }
    }
    Ok(Signature::new(r, s.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockRng {
        ret: U256
    }

    impl MockRng {
        fn new(ret: U256) -> Self {
            MockRng { ret }
        }
    }

    impl RandU256 for MockRng {
        fn gen_u256_range(&mut self, _: &U256, _: &U256) -> U256 {self.ret}
        fn gen_u256(&mut self) -> U256 {self.ret}
    }

    #[test]
    fn test_sign_message() -> Result <(), Errors>{
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let mut rng = MockRng::new(U256::one());
        let r = U256::from_dec_str("55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap();
        let s = U256::from_dec_str("46992992609640467017876430891725436310416382799367731925940059381452197848073").unwrap();
        assert_eq!(sign_message(&mut rng, U256::from(1), b"test", &secp256k1)?, Signature::from((r, s)));
        let mut rng = MockRng::new(r);
        let r = U256::from_dec_str("41368939038460017089690463593392860417892426308765457203329747030588589193225").unwrap();
        let s = U256::from_dec_str("31053304894587659915590992603088785556232634551332211685482548454257155309727").unwrap();
        assert_eq!(sign_message(&mut rng, U256::from(1), b"test", &secp256k1)?, Signature::from((r, s)));
        let mut rng = MockRng::new(s);
        let r = U256::from_dec_str("77932200203109347009278505663038934845025051719260868904944445924183195568966").unwrap();
        let s = U256::from_dec_str("18814421266250684974379844796716226997450433558338117269037970871169669132622").unwrap();
        assert_eq!(sign_message(&mut rng, U256::from(1), b"test", &secp256k1)?, Signature::from((r, s)));
        Ok(())
    }
}
