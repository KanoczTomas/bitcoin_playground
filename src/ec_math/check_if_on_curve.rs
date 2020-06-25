use crate::types::{U512, Points, EllipticCurve, ECpoint, Errors};
use crate::group_math::a_inverse_mod;

///Returns Some(ECpoint) if the point lies on the curve None otherwise
pub fn check_if_on_curve(p: Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    match p {
        Points::Infinity => Ok(ECpoint::Infinity),
        Points::FinitePoint(point) => {
            //y^2 = x^3 + ax + b
            let x = U512::from(point.x);
            let y = U512::from(point.y);
            let p = U512::from(curve.p);
            let a = U512::from(curve.a);
            let b = U512::from(curve.b);
            let y_2 = (y * y) % p;
            let x_3 = (((x * x) % p ) * x) % p;
            let minus_x3 = U512::from(a_inverse_mod(x_3.into(), p.into())?);
            let ax = (a * x) % p;
            let minus_ax = U512::from(a_inverse_mod(ax.into(), p.into())?);
            let minus_b = U512::from(a_inverse_mod(b.into(), p.into())?);
            let check_equation = (y_2 + minus_x3 + minus_ax + minus_b) % p;
            match  check_equation == U512::zero() {
                true => Ok(ECpoint::OnCurve(point)),
                false => Err(Errors::PointNotOnCurve(point))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{U256, EllipticCurve, Point};

    #[test]
    fn test_check_if_on_curve() {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let g = Point::new(secp256k1.g.0, secp256k1.g.1);
        let g1 = Point::new(secp256k1.g.0 + U256::one(), secp256k1.g.1 + U256::one());
        //base point is on curve
        assert_eq!(check_if_on_curve(Points::FinitePoint(g), &secp256k1), Ok(ECpoint::OnCurve(g)));
        //base point +1 is not on curve
        assert_eq!(check_if_on_curve(Points::FinitePoint(g1), &secp256k1), Err(Errors::PointNotOnCurve(g1)));
        //infinity is on curve
        assert_eq!(check_if_on_curve(Points::Infinity, &secp256k1), Ok(ECpoint::Infinity));
        // point (0,0) is not on curve
        let z = Point::new(U256::zero(), U256::zero());
        assert_eq!(check_if_on_curve(Points::FinitePoint(z), &secp256k1), Err(Errors::PointNotOnCurve(z)));
        let z = Point::new(U256::from(2), U256::zero());
        assert_eq!(check_if_on_curve(Points::FinitePoint(z), &secp256k1), Err(Errors::PointNotOnCurve(z)));
    }
}
