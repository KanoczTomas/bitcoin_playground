use crate::types::{U256, Points, EllipticCurve, ECpoint, Errors};
use crate::ec_math::{point_add, check_if_on_curve};

///Returns k * point computed by the double and point_add algorithm
pub fn scalar_mult(k: U256, point: &Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    let point = check_if_on_curve(*point, curve)?;
    if k % curve.n == U256::zero() {
        return Ok(ECpoint::Infinity)
    }
    if point == ECpoint::Infinity {
        return Ok(ECpoint::Infinity)
    }
    else {
        let mut result = ECpoint::Infinity;
        let mut addend = point;
        let mut bits = k;
        while bits != U256::zero() {
            if bits & U256::one() == U256::one(){
                //Add
                let result_as_point: Points = result.into();
                let addend_as_point: Points = addend.into();
                result = point_add(&result_as_point, &addend_as_point, curve)?;
            }
            //Double
            let addend_as_point: Points = addend.into();
            addend = point_add(&addend_as_point, &addend_as_point, curve)?;
            bits = bits >> 1;
        }
        let result = check_if_on_curve(result.into(), curve)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Point;
    #[test]
    fn test_scalar_mult() {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        #[allow(non_snake_case)]
        let G = Points::FinitePoint(Point::from(secp256k1.g));
        let k = U256::zero();
        let p = Points::Infinity;
        //0 * 0 = 0
        assert_eq!(scalar_mult(k, &p, &secp256k1), Ok(ECpoint::Infinity));
        let p = Point::new(secp256k1.g.0, secp256k1.g.1);
        //0 * p = 0
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::Infinity));
        //curve.n * p = 0
        assert_eq!(scalar_mult(secp256k1.n, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::Infinity));
        //1 * p = p
        assert_eq!(scalar_mult(U256::one(), &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(p)));
        let k = U256::from_dec_str("2").unwrap();
        let result_x = U256::from_dec_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap();
        let result_y = U256::from_dec_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap();
        let result = Point::new(result_x, result_y);
        //2 * G = result
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let k = U256::from_dec_str("255").unwrap();
        let result_x = U256::from_dec_str("12312385769684547396095365029355369071957339694349689622296638024179682296192").unwrap();
        let result_y = U256::from_dec_str("29045073188889159330506972844502087256824914692696728592611344825524969277689").unwrap();
        let result = Point::new(result_x, result_y);
        //255 * G = result
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let k = U256::from_dec_str("12312385769684547396095365029355369071957339694349689622296638024179682296192").unwrap();
        let result_x = U256::from_dec_str("107431185289838427080855157233861978627665866704688032938293294398756895973759").unwrap();
        let result_y = U256::from_dec_str("82111623719113235063168576279035646362600822088127451394020515078876578385407").unwrap();
        let result = Point::new(result_x, result_y);
        //12312385769684547396095365029355369071957339694349689622296638024179682296192 * G
        assert_eq!(scalar_mult(k, &Points::FinitePoint(Point::from(secp256k1.g)), &secp256k1), Ok(ECpoint::OnCurve(result)));
        //max 256 bit number * G
        let max = U256::from_dec_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap();
        let result_x = U256::from_dec_str("65766924097070208376629306902125118242069746467871217785643147593192657258159").unwrap();
        let result_y = U256::from_dec_str("109236945745669593534474897756172178689381177381602435107906663179476813370855").unwrap();
        let result = Point::new(result_x, result_y);
        assert_eq!(scalar_mult(max, &G, &secp256k1), Ok(ECpoint::OnCurve(result)));
    }
}
