#![feature(try_trait)]
extern crate bigint;
use bigint::uint;
use hex;
use rand;

#[allow(non_camel_case_types)]
type u256 = uint::U256;
#[allow(non_camel_case_types)]
type u512 = uint::U512;

#[derive(Debug,PartialEq,Clone,Copy)]
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

impl std::convert::From<(u256, u256)> for Point {
    fn from(t: (u256, u256)) -> Self {
        Point::new(t.0, t.1)
    }
}

#[derive(Debug, PartialEq,Clone,Copy)]
enum Points{
    Infinity,
    FinitePoint(Point)
}

impl std::convert::From<ECpoint> for Points {
    fn from(p: ECpoint)-> Self {
        match p {
            ECpoint::Infinity => Points::Infinity,
            ECpoint::OnCurve(p) => Points::FinitePoint(p)
        }
    }
}


#[derive(Debug,PartialEq,Clone,Copy)]
enum ECpoint {
    Infinity,
    OnCurve(Point),
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
    ZeroModulo,
    ///Point not on curve
    PointNotOnCurve(Point),
    ///Negative point not on curve(only happens if point_neg is buggy)
    NegativePointNotOnCurve(Point),
}



///Returns the additive inverse of k modulo p
///This function returns the only integer x such that (x + k) % p == 0
fn a_inverse_mod(k: u256, p: u256) -> Result<u256, Errors> {
    if k == u256::zero(){
        return Ok(k);
    }
    if p == u256::zero() {
        return Err(Errors::ZeroModulo);
    }
    //we deal with k > p by taking its reminder
    let k = k % p;
    let x = p - k;
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
        r = a_inverse_mod((quotient.full_mul(r) % u512::from(p)).into(), p)?;
        r = u256::from((u512::from(old_r) + u512::from(r)) % u512::from(p));
        old_r = tmp;
        tmp = s;
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

///Returns Some(ECpoint) if the point lies on the curve None otherwise
fn check_if_on_curve(p: Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    match p {
        Points::Infinity => Ok(ECpoint::Infinity),
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
            let check_equation = (y_2 + minus_x3 + minus_ax + minus_b) % p;
            match  check_equation == u512::zero() {
                true => Ok(ECpoint::OnCurve(point)),
                false => Err(Errors::PointNotOnCurve(point))
            }
        }
    }
}

///Returns -point or Errors
fn point_neg(p: Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
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
///Returns the result of point1 + point2 on curve according to the group law.
// fn ec_point_add(point1: &ECpoint, point2: &ECpoint, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
fn point_add(point1: &Points, point2: &Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    let point1 = check_if_on_curve(*point1, curve)?;
    let point2 = check_if_on_curve(*point2, curve)?;
    match (point1, point2) {
        (ECpoint::Infinity, ECpoint::Infinity) => return Ok(ECpoint::Infinity),
        (ECpoint::Infinity, ECpoint::OnCurve(p2)) => return Ok(ECpoint::OnCurve(p2)),
        (ECpoint::OnCurve(p1), ECpoint::Infinity) => return Ok(ECpoint::OnCurve(p1)),
        (ECpoint::OnCurve(p1), ECpoint::OnCurve(p2)) => {
            let Point { x: x1, y: y1} = p1;
            let Point { x: x2, y: y2} = p2;
            let x1 = u512::from(x1);
            let x2 = u512::from(x2);
            let y1 = u512::from(y1);
            let y2 = u512::from(y2);
            let minus_x1 = u512::from(a_inverse_mod(x1.into(), curve.p)?);
            let minus_x2 = u512::from(a_inverse_mod(x2.into(), curve.p)?);
            let minus_y2 = u512::from(a_inverse_mod(y2.into(), curve.p)?);
            // let minus_y1 = u512::from(a_inverse_mod(y1.into(), curve.p)?);
            if x1 == x2 && y1 != y2 {
                //point +(-point) = 0
                return Ok(ECpoint::Infinity);
            }
            let m: u512;
            if x1 == x2 {
                //point1 == point2
                // m = (3 * x1 * x1 + curve.a) * inverse_mod(2 * y1, curve.p)
                let x1_2 = (x1 * x1) % u512::from(curve.p);
                let x1_2_times_3: u512 = (u512::from(3) * x1_2) % u512::from(curve.p);
                let x1_2_times_3_plus_a = (x1_2_times_3 + u512::from(curve.a)) % u512::from(curve.p);
                let y1_times_2 = (y1 * 2.into()) % u512::from(curve.p);
                let inverse_y1_times_2 = u512::from(m_inverse_mod(u256::from(y1_times_2), curve.p)?);
                m = x1_2_times_3_plus_a * inverse_y1_times_2;
            }
            else {
                //This is the case point1 != point2.
                // m = (y1 - y2) * inverse_mod(x1 - x2, curve.p)
                // m =
                //     (y1 + minus_y2)
                //     * u512::from(m_inverse_mod(u256::from((x1 + minus_x2) % u512::from(curve.p)), curve.p)?);
                let y1_minus_y2 = (y1 + minus_y2) % u512::from(curve.p);
                let x1_minus_x2 = (x1 + minus_x2) % u512::from(curve.p);
                let inverse_x1_minus_x2 = u512::from(m_inverse_mod(x1_minus_x2.into(), curve.p)?);
                m = y1_minus_y2 * inverse_x1_minus_x2;
            }
            let m = m % u512::from(curve.p);
            let x3 = ((m * m) + minus_x1 + minus_x2) % u512::from(curve.p);
            let y3 = (y1 + m * ((x3 + minus_x1) % u512::from(curve.p))) % u512::from(curve.p);
            let minux_y3 = u512::from(a_inverse_mod(y3.into(), curve.p)?);
            Ok(ECpoint::OnCurve(Point::new(x3.into(), minux_y3.into())))
        }
    }
}

///Returns k * point computed by the double and point_add algorithm
fn scalar_mult(k: u256, point: &Points, curve: &EllipticCurve) -> Result<ECpoint, Errors> {
    let point = check_if_on_curve(*point, curve)?;
    if k % curve.n == u256::zero() {
        return Ok(ECpoint::Infinity)
    }
    if point == ECpoint::Infinity {
        return Ok(ECpoint::Infinity)
    }
    else {
        let mut result = ECpoint::Infinity;
        let mut addend = point;
        let mut bits = k;
        while bits != u256::zero() {
            if bits & u256::one() == u256::one(){
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

fn generate_random_u256() -> u256 {
    let mut bytes = [0u8; 32];
    for byte in bytes.iter_mut() {
        *byte = rand::random::<u8>();
    }
    bytes.into()
}

///Generates a random prive-public key pair.
fn make_keypair(curve: &EllipticCurve) -> Result<(u256, ECpoint), Errors> {
    let mut private_key;
    loop {
        private_key = generate_random_u256();
        if private_key >= u256::one() && private_key < curve.n {
            break;
        }
    }
    let public_key = scalar_mult(private_key, &Points::FinitePoint(Point::from(curve.g)), curve)?;
    println!("public_key = {:?}", public_key);
    Ok((private_key, public_key))

}

///Constructs secp256k1 EllipticCurve
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
    println!("{:?} is {}", &p,
        check_if_on_curve(Points::FinitePoint(p), &secp256k1)
        // .ok_or_else(|| Errors::PointNotOnCurve(p))
        .map(|_| "on curve")
        .unwrap_or("not on curve")
    );
    let p = Point::new(
        u256::from_big_endian(&hex::decode("2dc502956364ac430fbe94cdd6bafda73b1b620b5fed00a813af5c5ea93cf73d").unwrap()),
        u256::from_big_endian(&hex::decode("72e1ee03ecd1d250a63a4795dd6998b26aeba68048ff8c1e5289bf976309aec1").unwrap())
    );
    println!("{:?} is {:?}", &p, check_if_on_curve(Points::FinitePoint(p), &secp256k1));
    println!("Negative of {:?} is {:?}", &p, point_neg(Points::FinitePoint(p), &secp256k1));
    let p = Point::new(secp256k1.g.0 + u256::one(), secp256k1.g.1 + u256::one());
    println!("{:?} is {}", &p, check_if_on_curve(Points::FinitePoint(p), &secp256k1).map(|_| "on curve").unwrap_or("not on curve"));
    println!("Negative of Infinity is {:?}", point_neg(Points::Infinity, &secp256k1));
    let p = Point::new(u256::zero(), u256::zero());
    println!("{:?} is {}", &p, check_if_on_curve(Points::FinitePoint(p),&secp256k1).map(|_| "on curve").unwrap_or("not on curve"));
    match m_inverse_mod(u256::from(2), u256::from(10)){
        Ok(x) => println!("{}", x),
        Err(err) => println!("{:?}", err)
    }
    let p1_x = u256::from_dec_str("93032511444448586572795960096940553314020690780422011061136711682476439908486").unwrap();
    let p1_y = u256::from_dec_str("24170782756704702697334930591920306786018768055625159525941195559726624089280").unwrap();
    let p1 = Point::new(p1_x, p1_y);
    let p2_x = u256::from_dec_str("59333657243042948346465692029809134503478384592765371424656931804815875295262").unwrap();
    let p2_y = u256::from_dec_str("93619890378675464164465240783457470719643004351985897479073520136131107550882").unwrap();
    let p2 = Point::new(p2_x, p2_y);
    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);
    println!("p1 + p2 = {:?}", point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1));
    println!("G = {:?}", Point::new(secp256k1.g.0, secp256k1.g.1));


    let k = u256::from_dec_str("2").unwrap();
    let result_x = u256::from_dec_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap();
    let result_y = u256::from_dec_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap();
    let result = Point::new(result_x, result_y);
    //2 * G = result
    let p = Point::new(secp256k1.g.0, secp256k1.g.1);
    assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(result)));
    println!("2 * G = {:?}", result);
    println!("Alice key pair: {:?}", make_keypair(&secp256k1)?);
    println!("Bob key pair: {:?}", make_keypair(&secp256k1)?);

    // check_if_on_curve(Points::FinitePoint(p), &secp256k1)?;
    // m_inverse_mod(u256::from(2), u256::from(10))?;
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
    fn test_check_if_on_curve() {
        let secp256k1 = secp256k1_factory();
        let g = Point::new(secp256k1.g.0, secp256k1.g.1);
        let g1 = Point::new(secp256k1.g.0 + 1.into(), secp256k1.g.1 + 1.into());
        //base point is on curve
        assert_eq!(check_if_on_curve(Points::FinitePoint(g), &secp256k1), Ok(ECpoint::OnCurve(g)));
        //base point +1 is not on curve
        assert_eq!(check_if_on_curve(Points::FinitePoint(g1), &secp256k1), Err(Errors::PointNotOnCurve(g1)));
        //infinity is on curve
        assert_eq!(check_if_on_curve(Points::Infinity, &secp256k1), Ok(ECpoint::Infinity));
        // point (0,0) is not on curve
        let z = Point::new(u256::zero(), u256::zero());
        assert_eq!(check_if_on_curve(Points::FinitePoint(z), &secp256k1), Err(Errors::PointNotOnCurve(z)));
        let z = Point::new(u256::from(2), u256::zero());
        assert_eq!(check_if_on_curve(Points::FinitePoint(z), &secp256k1), Err(Errors::PointNotOnCurve(z)));
    }
    #[test]
    fn test_point_neg() {
        let secp256k1 = secp256k1_factory();
        let g = Point::new(secp256k1.g.0, secp256k1.g.1);
        let g1 = Point::new(secp256k1.g.0 + 1.into(), secp256k1.g.1 + 1.into());
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
    #[test]
    fn test_point_add() -> Result<(), Errors> {
        let secp256k1 = secp256k1_factory();
        let p1 = Points::Infinity;
        let p2 = Points::Infinity;
        //0 + 0 = 0
        assert_eq!(point_add(&p1, &p2, &secp256k1), Ok(ECpoint::Infinity));
        let p2 = Point::new(secp256k1.g.0, secp256k1.g.1);
        //0 + p2 = p2
        assert_eq!(point_add(&p1, &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(p2)));
        let p1_x = u256::from_dec_str("14724152641787391886706825019140647642831456942302888313454887185756386459261").unwrap();
        let p1_y = u256::from_dec_str("33606834801707400867022010659444293494773240287788223067571598517486946424308").unwrap();
        let p1 = Point::new(p1_x, p1_y);
        //p1 + 0 = p1
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::Infinity, &secp256k1), Ok(ECpoint::OnCurve(p1)));
        let p2_x = p1_x;
        let p2_y = a_inverse_mod(p1_y, secp256k1.p)?;
        let p2 = Point::new(p2_x, p2_y);
        //p1 + (-p1) = 0
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::Infinity));
        let p2 = Point::new(p2_x, p2_y + 1.into());
        //p1 + p2 (not on curve) => Should return Errors::PointNotOnCurve(p2)
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Err(Errors::PointNotOnCurve(p2)));
        let p1_x = u256::from_dec_str("93032511444448586572795960096940553314020690780422011061136711682476439908486").unwrap();
        let p1_y = u256::from_dec_str("24170782756704702697334930591920306786018768055625159525941195559726624089280").unwrap();
        let p1 = Point::new(p1_x, p1_y);
        let p2_x = u256::from_dec_str("59333657243042948346465692029809134503478384592765371424656931804815875295262").unwrap();
        let p2_y = u256::from_dec_str("93619890378675464164465240783457470719643004351985897479073520136131107550882").unwrap();
        let p2 = Point::new(p2_x, p2_y);
        let result_x = u256::from_dec_str("47190285491955357084468366023854200066944523930992042921712053222486252941719").unwrap();
        let result_y = u256::from_dec_str("96761708245943761358849560795104005001889126796688928595545898056024327671746").unwrap();
        let result = Point::new(result_x, result_y);
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(result)));
        //tet p2 + p1 = result
        assert_eq!(point_add(&Points::FinitePoint(p1), &Points::FinitePoint(p2), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let g1 = Points::FinitePoint(Point::from(secp256k1.g));
        let result_x = u256::from_dec_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap();
        let result_y = u256::from_dec_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap();
        let result = Point::new(result_x, result_y);
        //test G + G = 2G
        assert_eq!(point_add(&g1, &g1, &secp256k1), Ok(ECpoint::OnCurve(result)));
        Ok(())
    }
    #[test]
    fn test_scalar_mult() {
        let secp256k1 = secp256k1_factory();
        let k = u256::zero();
        let p = Points::Infinity;
        //0 * 0 = 0
        assert_eq!(scalar_mult(k, &p, &secp256k1), Ok(ECpoint::Infinity));
        let p = Point::new(secp256k1.g.0, secp256k1.g.1);
        //0 * p = 0
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::Infinity));
        //curve.n * p = 0
        assert_eq!(scalar_mult(secp256k1.n, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::Infinity));
        //1 * p = p
        assert_eq!(scalar_mult(u256::one(), &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(p)));
        let k = u256::from_dec_str("2").unwrap();
        let result_x = u256::from_dec_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").unwrap();
        let result_y = u256::from_dec_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").unwrap();
        let result = Point::new(result_x, result_y);
        //2 * G = result
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let k = u256::from_dec_str("255").unwrap();
        let result_x = u256::from_dec_str("12312385769684547396095365029355369071957339694349689622296638024179682296192").unwrap();
        let result_y = u256::from_dec_str("29045073188889159330506972844502087256824914692696728592611344825524969277689").unwrap();
        let result = Point::new(result_x, result_y);
        //255 * G = result
        assert_eq!(scalar_mult(k, &Points::FinitePoint(p), &secp256k1), Ok(ECpoint::OnCurve(result)));
        let k = u256::from_dec_str("12312385769684547396095365029355369071957339694349689622296638024179682296192").unwrap();
        let result_x = u256::from_dec_str("107431185289838427080855157233861978627665866704688032938293294398756895973759").unwrap();
        let result_y = u256::from_dec_str("82111623719113235063168576279035646362600822088127451394020515078876578385407").unwrap();
        let result = Point::new(result_x, result_y);
        //12312385769684547396095365029355369071957339694349689622296638024179682296192 * G
        assert_eq!(scalar_mult(k, &Points::FinitePoint(Point::from(secp256k1.g)), &secp256k1), Ok(ECpoint::OnCurve(result)));
    }
    #[test]
    fn test_generate_random_u256() {
        let num = generate_random_u256();
        match num {
            bigint::U256(_) => {},
            _ => panic!("return value should be u256")
        }
    }
    #[test]
    fn test_make_keypair() -> Result<(), Errors>{
        let secp256k1 = secp256k1_factory();
        let (private_key, public_key) = make_keypair(&secp256k1)?;
        assert_eq!(public_key, scalar_mult(private_key, &Points::FinitePoint(Point::from(secp256k1.g)), &secp256k1)?);
        Ok(())
    }
}
