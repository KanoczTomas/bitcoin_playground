extern crate bigint;
use bigint::uint;
use hex;

#[allow(non_camel_case_types)]
type u256 = uint::U256;

#[derive(Debug)]
struct NonZeroECpoint {
    x: u256,
    y: u256
}

impl NonZeroECpoint {
    fn new(_x: u256, _y: u256) -> Self {
        Self {
            x: _x,
            y: _y
        }
    }
}

#[derive(Debug)]
enum ECpoint {
    ///reprezents the zero point
    Zero,
    ///all other EC points
    Point(NonZeroECpoint)
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
    pub fn p(&mut self, _p: &str) -> &mut Self{
        self.p = Self::pick_hex_or_dec(_p);
        self
    }
    pub fn a(&mut self, _a: i64) -> &mut Self{
        self.a = _a;
        self
    }
    pub fn b(&mut self, _b: i64) -> &mut Self{
        self.b = _b;
        self
    }
    pub fn g(&mut self, _g: (&str, &str)) -> &mut Self{
        self.g = (Self::pick_hex_or_dec(_g.0), Self::pick_hex_or_dec(_g.1));
        self
    }
    pub fn n(&mut self, _n: &str) -> &mut Self{
        self.n = Self::pick_hex_or_dec(_n);
        self
    }
    pub fn h(&mut self, _h: &str) -> &mut Self{
        self.h = Self::pick_hex_or_dec(_h);
        self
    }
}

#[derive(Debug)]
enum Errors {
    /// zero division error
    ZeroDivision,
    ///The number is not a multiplicative inverse mod p
    NoMultiplicativeInverse(u256,u256)
}

///Returns the additive inverse of k modulo p
///This function returns the only integer x such that (x + k) % p == 0
fn a_inverse_mod(k: u256, p: u256) -> u256 {
    //we deal with k > p by taking its reminder
    let k = k % p;
    let x = p - k;
    assert_eq!((x + k) % p, u256::from(0));
    x
}

///Returns the inverse of k modulo p.
///This function returns the only integer x such that (x * k) % p == 1.
///k must be non-zero and p must be a prime.
fn m_inverse_mod(k: u256, p: u256) -> Result<u256, Errors>{
    if k == 0.into() {
        return Err(Errors::ZeroDivision);
    }
    //Extended Euclidean algorithm.
    let (mut s, mut old_s) = (u256::from(0), u256::from(1));
    let (mut r, mut old_r) = (p, k);
    while r != u256::from(0) {
        let quotient = old_r / r;
        let mut tmp = r;
        // r = old_r - quotient * r;
        r = (old_r + a_inverse_mod(quotient * r, p)) % p;
        old_r = tmp;
        tmp = s;
        // s = old_s - quotient * s;
        s = (old_s + a_inverse_mod(quotient * s, p)) % p;
        old_s = tmp;
    }
    let (gcd, x) = (old_r, old_s);
    if gcd != u256::from(1) {
        return Err(Errors::NoMultiplicativeInverse(k,p));
    }
    assert_eq!((x * k) % p, u256::from(1));
    Ok(x)
}

///Return true if the point lies on the curve
fn is_on_curve(p: ECpoint, curve: &EllipticCurve) -> bool {
    match p {
        ECpoint::Zero => true,
        ECpoint::Point(p) => {
            (p.y * p.y - p.x * p.x * p.x - u256::from(curve.a) * p.x - u256::from(curve.b)) % curve.p == u256::from(0)
        }
    }
}

fn main() {
    let mut secp256k1 = EllipticCurve::new("secp256k1");
    secp256k1
    .p("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f")
    // Curve coefficients.
    .a(0)
    .b(7)
    // Base point. (a tupple)
    .g(("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"))
    // Subgroup order.
    .n("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141")
    // Subgroup cofactor.
    .h("1");

    println!("{:?}", &secp256k1);

    match m_inverse_mod(u256::from(2), u256::from(10)) {
        Ok(x) => println!("{}", x),
        Err(err) => println!("{:?}", err)
    }
}
