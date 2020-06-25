use crate::types::{U256, U512, Point, Points, ECpoint, Errors, EllipticCurve};
use crate::group_math::{a_inverse_mod, m_inverse_mod};
use crate::ec_math::{check_if_on_curve, point_neg};

///Returns the result of point1 + point2 on curve according to the group law.
pub fn point_add(point1: &Points, point2: &Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    let point1 = check_if_on_curve(*point1, curve)?;
    let point2 = check_if_on_curve(*point2, curve)?;
    match (point1, point2) {
        (ECpoint::Infinity, ECpoint::Infinity) => Ok(ECpoint::Infinity),
        (ECpoint::Infinity, ECpoint::OnCurve(p2)) => Ok(ECpoint::OnCurve(p2)),
        (ECpoint::OnCurve(p1), ECpoint::Infinity) => Ok(ECpoint::OnCurve(p1)),
        (ECpoint::OnCurve(p1), ECpoint::OnCurve(p2)) => {
            let Point { x: x1, y: y1} = p1;
            let Point { x: x2, y: y2} = p2;
            let x1 = U512::from(x1);
            let x2 = U512::from(x2);
            let y1 = U512::from(y1);
            let y2 = U512::from(y2);
            let minus_x1 = U512::from(a_inverse_mod(x1.into(), curve.p)?);
            let minus_x2 = U512::from(a_inverse_mod(x2.into(), curve.p)?);
            let minus_y2 = U512::from(a_inverse_mod(y2.into(), curve.p)?);
            if x1 == x2 && y1 != y2 {
                //point +(-point) = 0
                return Ok(ECpoint::Infinity);
            }
            let m: U512;
            if x1 == x2 {
                //point1 == point2
                // m = (3 * x1 * x1 + curve.a) * inverse_mod(2 * y1, curve.p)
                let x1_2 = (x1 * x1) % U512::from(curve.p);
                let x1_2_times_3: U512 = (U512::from(3) * x1_2) % U512::from(curve.p);
                let x1_2_times_3_plus_a = (x1_2_times_3 + U512::from(curve.a)) % U512::from(curve.p);
                let y1_times_2: U512 = (y1 * U512::from(2)) % U512::from(curve.p);
                let inverse_y1_times_2 = U512::from(m_inverse_mod(U256::from(y1_times_2), curve.p)?);
                m = x1_2_times_3_plus_a * inverse_y1_times_2;
            }
            else {
                //This is the case point1 != point2.
                // m = (y1 - y2) * inverse_mod(x1 - x2, curve.p)
                let y1_minus_y2 = (y1 + minus_y2) % U512::from(curve.p);
                let x1_minus_x2 = (x1 + minus_x2) % U512::from(curve.p);
                let inverse_x1_minus_x2 = U512::from(m_inverse_mod(x1_minus_x2.into(), curve.p)?);
                m = y1_minus_y2 * inverse_x1_minus_x2;
            }
            let m = m % U512::from(curve.p);
            let x3 = ((m * m) + minus_x1 + minus_x2) % U512::from(curve.p);
            let y3 = (y1 + m * ((x3 + minus_x1) % U512::from(curve.p))) % U512::from(curve.p);
            Ok(point_neg(Points::FinitePoint(Point::from((x3.into(), y3.into()))), curve)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() -> Result<(), Errors> {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        let p1 = Points::Infinity;
        let p2 = Points::Infinity;
        //0 + 0 = 0
        assert_eq!(point_add(&p1, &p2, &secp256k1), Ok(ECpoint::Infinity));
        let p2 = Point::new(secp256k1.g.0, secp256k1.g.1);
        //0 + p2 = p2
        assert_eq!(point_add(&p1, &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(p2)));
        let p1_x = U256::from_dec_str("14724152641787391886706825019140647642831456942302888313454887185756386459261").unwrap();
        let p1_y = U256::from_dec_str("33606834801707400867022010659444293494773240287788223067571598517486946424308").unwrap();
        let p1 = Point::new(p1_x, p1_y);
        //p1 + 0 = p1
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::Infinity, &secp256k1), Ok(ECpoint::OnCurve(p1)));
        let p2_x = p1_x;
        let p2_y = a_inverse_mod(p1_y, secp256k1.p)?;
        let p2 = Point::new(p2_x, p2_y);
        //p1 + (-p1) = 0
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::Infinity));
        let p2 = Point::new(p2_x, p2_y + U256::one());
        //p1 + p2 (not on curve) => Should return Errors::PointNotOnCurve(p2)
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Err(Errors::PointNotOnCurve(p2)));
        let p1_x = U256::from_dec_str("93032511444448586572795960096940553314020690780422011061136711682476439908486").unwrap();
        let p1_y = U256::from_dec_str("24170782756704702697334930591920306786018768055625159525941195559726624089280").unwrap();
        let p1 = Point::new(p1_x, p1_y);
        let p2_x = U256::from_dec_str("59333657243042948346465692029809134503478384592765371424656931804815875295262").unwrap();
        let p2_y = U256::from_dec_str("93619890378675464164465240783457470719643004351985897479073520136131107550882").unwrap();
        let p2 = Point::new(p2_x, p2_y);
        let result_x = U256::from_dec_str("47190285491955357084468366023854200066944523930992042921712053222486252941719").unwrap();
        let result_y = U256::from_dec_str("96761708245943761358849560795104005001889126796688928595545898056024327671746").unwrap();
        let result = Point::new(result_x, result_y);
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(result)));
        //tet p2 + p1 = result
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let g1 = Points::FinitePoint(Point::from(secp256k1.g));
        let result_x = U256::from_dec_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap();
        let result_y = U256::from_dec_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap();
        let result = Point::new(result_x, result_y);
        //test G + G = 2G
        assert_eq!(point_add(&g1, &g1, &secp256k1), Ok(ECpoint::OnCurve(result)));
        Ok(())
    }
}
