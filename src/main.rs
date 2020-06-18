#![feature(try_trait)]
extern crate bigint;
use bigint::uint;
use hex;

#[allow(non_camel_case_types)]
type u256 = uint::U256;
#[allow(non_camel_case_types)]
type u512 = uint::U512;

#[derive(Debug,PartialEq)]
struct Point {
    x: u256,
    y: u256
}

impl Point {
    fn new(_x: u256, _y: u256) -> Self {
        Self {
            x: _x,
            y: _y
        }
    }
}

#[derive(Debug, PartialEq)]
enum Points<'a> {
    Infinity,
    FinitePoint(&'a Point)
}

#[derive(Debug,PartialEq)]
enum ECpoint<'a> {
    Infinity,
    OnCurve(&'a Point),
}




#[derive(Debug)]
struct EllipticCurve {
    ///name of the curve
    name: String,
    ///prime (modulo)
    p: u256,
    ///a coefficient
    a: i64,
    ///b coefficient
    b: i64,
    ///base point
    g: (u256, u256),
    ///subgroup order
    n: u256,
    ///subgroup cofactor
    h: u256
}

impl EllipticCurve {
    // if a str is given with 0x it will treat it as hexa string otherwise a decadic number
    fn pick_hex_or_dec(s: &str) -> u256{
        match s.starts_with("0x"){
            true => u256::from_big_endian(&hex::decode(s.trim_start_matches("0x")).unwrap()),
            false => u256::from_dec_str(s).unwrap()
        }
    }
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            p: u256::default(),
            a: i64::default(),
            b: i64::default(),
            g: (u256::default(),
                u256::default()),
            n: u256::default(),
            h: u256::default(),
        }
    }
    pub fn set_p(&mut self, _p: &str) -> &mut Self{
        self.p = Self::pick_hex_or_dec(_p);
        self
    }
    pub fn set_a(&mut self, _a: i64) -> &mut Self{
        self.a = _a;
        self
    }
    pub fn set_b(&mut self, _b: i64) -> &mut Self{
        self.b = _b;
        self
    }
    pub fn set_g(&mut self, _g: (&str, &str)) -> &mut Self{
        self.g = (Self::pick_hex_or_dec(_g.0), Self::pick_hex_or_dec(_g.1));
        self
    }
    pub fn set_n(&mut self, _n: &str) -> &mut Self{
        self.n = Self::pick_hex_or_dec(_n);
        self
    }
    pub fn set_h(&mut self, _h: &str) -> &mut Self{
        self.h = Self::pick_hex_or_dec(_h);
        self
    }
}

#[derive(Debug,PartialEq)]
enum Errors {
    ///Zero division error
    ZeroDivision,
    ///The number is not a multiplicative inverse mod p
    NoMultiplicativeInverse(u256,u256),
    ///Zero modulo error
    ZeroModulo
}



///Returns the additive inverse of k modulo p
///This function returns the only integer x such that (x + k) % p == 0
// fn a_inverse_mod(k: u256, p: u256) -> u256 {
fn a_inverse_mod(k: u256, p: u256) -> Result<u256, Errors> {
    if k == u256::zero(){
        return Ok(k);
    }
    // if p == u256::zero() {
    //     panic!("call a_inverse_mod({}, {}) => modulo 0 is undefined!",k ,p);
    // }
    if p == u256::zero() {
        return Err(Errors::ZeroModulo);
    }
    //we deal with k > p by taking its reminder
    let k = k % p;
    let x = p - k;
    // assert_eq!((x + k) % p, u256::from(0));
    Ok(x)
}

///Returns the multiplicative inverse of k modulo p.
///This function returns the only integer x such that (x * k) % p == 1.
///k must be non-zero and p must be a prime.
fn m_inverse_mod(k: u256, p: u256) -> Result<u256, Errors>{
    if k == 0.into(){
        return Err(Errors::ZeroDivision);
    }
    if p == 0.into() {
        return Err(Errors::ZeroModulo)
    }
    //Extended Euclidean algorithm.
    let (mut s, mut old_s) = (u256::from(0), u256::from(1));
    let (mut r, mut old_r) = (p, k);
    while r != u256::from(0) {
        let quotient = old_r / r;
        let mut tmp = r;
        // r = old_r - quotient * r;
        // println!("debug: => {}",quotient.full_mul(r));
        // println!("debug: => {}",(quotient.full_mul(r) % u512::from(p)));
        // println!("debug: => {:?}",a_inverse_mod((quotient.full_mul(r) % u512::from(p)).into(), p)? );
        // r = (old_r + a_inverse_mod((quotient.full_mul(r) % u512::from(p)).into(), p)?) % p;
        r = a_inverse_mod((quotient.full_mul(r) % u512::from(p)).into(), p)?;
        r = u256::from((u512::from(old_r) + u512::from(r)) % u512::from(p));

        old_r = tmp;
        tmp = s;
        // s = old_s - quotient * s;
        // s = (old_s + a_inverse_mod((quotient.full_mul(s) % u512::from(p)).into(), p)?) % p;
        s = a_inverse_mod((quotient.full_mul(s) % u512::from(p)).into(), p)?;
        s = u256::from((u512::from(old_s) + u512::from(s)) % u512::from(p));
        old_s = tmp;
    }
    let (gcd, x) = (old_r, old_s);
    if gcd != u256::from(1) {
        return Err(Errors::NoMultiplicativeInverse(k,p));
    }
    Ok(x)
}

///Return true if the point lies on the curve
///None represents the point at infinity
fn is_on_curve<'a>(p: &'a Points, curve: &EllipticCurve) -> Option<ECpoint<'a>> {
    match p {
        Points::Infinity => Some(ECpoint::Infinity),
        Points::FinitePoint(point) => {
            //y^2 = x^3 + ax + b
            let x = u512::from(point.x);
            let y = u512::from(point.y);
            let p = u512::from(curve.p);
            let a = u512::from(curve.a);
            let b = u512::from(curve.b);
            let y_2 = (y * y) % p;
            let x_3 = (((x * x) % p ) * x) % p;
            let minus_x3 = u512::from(a_inverse_mod(x_3.into(), p.into()).ok().unwrap());
            let ax = (a * x) % p;
            let minus_ax = u512::from(a_inverse_mod(ax.into(), p.into()).ok().unwrap());
            let minus_b = u512::from(a_inverse_mod(b.into(), p.into()).ok().unwrap());
            match (y_2 + minus_x3 + minus_ax + minus_b) % p == u512::zero() {
                true => Some(ECpoint::OnCurve(point)),
                false => None
            }
        }
    }
}

///Returns -point.
///None represents point at infinity
// fn point_neg(p: Option<&ECpoint>, curve: &EllipticCurve) -> Option<ECpoint> {
//     assert_eq!(is_on_curve(p, curve), PointInfo::OnCurve);
//     match p {
//         None => None,
//         Some(p) => {
//             let result = ECpoint::new(p.x, a_inverse_mod(p.y, curve.p));
//             assert_eq!(is_on_curve(Some(&result), curve), PointInfo::OnCurve);
//             Some(result)
//         }
//     }
// }

fn secp256k1_factory() -> EllipticCurve {
    let mut secp256k1 = EllipticCurve::new("secp256k1");
    secp256k1
    .set_p("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f")
    // Curve coefficients.
    .set_a(0)
    .set_b(7)
    // Base point. (a tupple)
    .set_g(("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"))
    // Subgroup order.
    .set_n("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141")
    // Subgroup cofactor.
    .set_h("1");
    secp256k1
}

fn main() -> Result<(), Errors> {
    let secp256k1 = secp256k1_factory();
    println!("{:?}", &secp256k1);
    let p = Point::new(secp256k1.g.0, secp256k1.g.1);
    // println!("{:?} is {:?}", &p, is_on_curve(Some(&p), &secp256k1));
    let p = Point::new(
        u256::from_big_endian(&hex::decode("2dc502956364ac430fbe94cdd6bafda73b1b620b5fed00a813af5c5ea93cf73d").unwrap()),
        u256::from_big_endian(&hex::decode("72e1ee03ecd1d250a63a4795dd6998b26aeba68048ff8c1e5289bf976309aec1").unwrap())
    );
    // println!("{:?} is {:?}", &p, is_on_curve(Some(&p), &secp256k1));
    // println!("Negative of {:?} is {:?}", &p, point_neg(Some(&p), &secp256k1));
    // let p = ECpoint::new(secp256k1.g.0 + u256::one(), secp256k1.g.1 + u256::one());
    // println!("{:?} is {:?}", &p, is_on_curve(Some(&p), &secp256k1));
    // println!("Negative of None is {:?}", point_neg(None, &secp256k1));
    // let p = ECpoint::new(u256::zero(), u256::zero());
    // println!("{:?} is {:?}", &p, is_on_curve(Some(&p),&secp256k1));

    // match m_inverse_mod(u256::from(2), u256::from(10)) {
    //     Ok(x) => println!("{}", x),
    //     Err(err) => println!("{:?}", err)
    // }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_inverse_mod() {
        let p = secp256k1_factory().p;
        let k = u256::from_dec_str("51962848049517897314481377586705320001209492118704192225945377961561169702593").unwrap();
        let x = u256::from_dec_str("63829241187798298109089607421982587852060492546936371813512206046347664969070").unwrap();
        assert_eq!(a_inverse_mod(k, p), Ok(x));
        assert_eq!((k + x) % p, u256::zero());
        let p = u256::from(11);
        let k = u256::from(5);
        let x = u256::from(6);
        assert_eq!(a_inverse_mod(k, p), Ok(x));
        assert_eq!((k + x) % p, u256::zero());
        let p = u256::from(0);
        assert_eq!(a_inverse_mod(k, p), Err(Errors::ZeroModulo));
        let p = u256::from(10);
        let k = u256::from(5);
        let x = u256::from(6);
        assert_ne!(a_inverse_mod(k, p), Ok(x));
        assert_ne!((k + x) % p, u256::zero());
    }
    #[test]
    fn test_m_inverse_mod() {
        //5 has no multiplicative inverse
        let p = u256::from(10);
        let k = u256::from(5);
        assert_eq!(m_inverse_mod(k, p), Err(Errors::NoMultiplicativeInverse(k,p)));
        //5 has 9 as multiplicative inverse
        let p = u256::from(11);
        let k = u256::from(5);
        let x = u256::from(9);
        assert_eq!(m_inverse_mod(k, p), Ok(x));
        assert_eq!((k * x) % p, u256::one());
        //finding the multiplicative inverse of 0 is not defined
        assert_eq!(m_inverse_mod(u256::zero(), p), Err(Errors::ZeroDivision));
        //additive inverse of k (x) is not the multiplicative inverse
        let p = secp256k1_factory().p;
        let k = u256::from_dec_str("51962848049517897314481377586705320001209492118704192225945377961561169702593").unwrap();
        let x = u256::from_dec_str("63829241187798298109089607421982587852060492546936371813512206046347664969070").unwrap();
        assert_ne!(m_inverse_mod(k, p), Ok(x));
        assert_ne!(k.full_mul(x) % u512::from(p), u512::one());
        //this should pass as x is the multiplicative inverse of k
        let x = u256::from_dec_str("15770621123931935841922866852148091009166141688620356011139719709837462056333").unwrap();
        assert_eq!(m_inverse_mod(k, p), Ok(x));
        assert_eq!(k.full_mul(x) % u512::from(p), u512::one())
    }
    #[test]
    fn test_is_on_curve() {
        let secp256k1 = secp256k1_factory();
        let g = Point::new(secp256k1.g.0, secp256k1.g.1);
        let g1 = Point::new(secp256k1.g.0 + 1.into(), secp256k1.g.1 + 1.into());
        //base point is on curve
        assert_eq!(is_on_curve(&Points::FinitePoint(&g), &secp256k1), Some(ECpoint::OnCurve(&g)));
        //base point +1 is not on curve
        assert_eq!(is_on_curve(&Points::FinitePoint(&g1), &secp256k1), None);
        //infinity is on curve
        assert_eq!(is_on_curve(&Points::Infinity, &secp256k1), Some(ECpoint::Infinity));
        // point (0,0) is not on curve
        let z = Point::new(u256::zero(), u256::zero());
        assert_eq!(is_on_curve(&Points::FinitePoint(&z), &secp256k1), None);
    }
}
