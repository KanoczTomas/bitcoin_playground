use crate::types::{Point, Points, EllipticCurve, ECpoint, Errors};
use crate::group_math::a_inverse_mod;
use crate::ec_math::check_if_on_curve;

///Returns the negative of p (x, -y) or Errors
pub fn point_neg(p: Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    match p {
        Points::Infinity => Ok(ECpoint::Infinity),
        Points::FinitePoint(p) => {
            match check_if_on_curve(Points::FinitePoint(p), curve){
                Ok(_) => {
                    let result = Point::new(p.x, a_inverse_mod(p.y, curve.p)?);
                    match check_if_on_curve(Points::FinitePoint(result), curve){
                        Ok(_) => Ok(ECpoint::OnCurve(result)),
                        Err(_) => Err(Errors::NegativePointNotOnCurve(result))
                    }
                },
                Err(_) => Err(Errors::PointNotOnCurve(p))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::U256;
    #[test]
    fn test_point_neg() {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let g = Point::new(secp256k1.g.0, secp256k1.g.1);
        let g1 = Point::new(secp256k1.g.0 + U256::one(), secp256k1.g.1 + U256::one());
        let y_inv = a_inverse_mod(g.y, secp256k1.p).unwrap();
        //negative of (g.x, g.y) is (g.x, -g.y)
        assert_eq!(point_neg(Points::FinitePoint(g), &secp256k1), Ok(ECpoint::OnCurve(Point::new(g.x, y_inv))));
        //g1 is not on curve so has no negative
        assert_eq!(point_neg(Points::FinitePoint(g1), &secp256k1), Err(Errors::PointNotOnCurve(g1)));
        //infinity is on curve and is its own negative
        assert_eq!(point_neg(Points::Infinity, &secp256k1), Ok(ECpoint::Infinity));
        //point (0, 0) is not on curve so has no negative
        let zero = Point::new(0.into(), 0.into());
        assert_eq!(point_neg(Points::FinitePoint(zero), &secp256k1), Err(Errors::PointNotOnCurve(zero)));
    }
}
