extern crate bigint;
use bigint::uint;
use bigint::FromDecStrErr;
use hex;

#[derive(Debug)]
struct EllipticCurve {
    ///name of the curve
    name: &'static str,
    ///prime (modulo)
    p: uint::U256,
    ///a coefficient
    a: i64,
    ///b coefficient
    b: i64,
    ///base point
    g: (uint::U256, uint::U256),
    ///subgroup order
    n: uint::U256,
    ///subgroup cofactor
    h: uint::U256
}

impl EllipticCurve {
    pub fn new(
        name: &'static str,
        p: &str,
        a: i64,
        b: i64,
        g: (&str, &str),
        n: &str,
        h: &str
    ) -> Self {
        Self {
            name: name,
            p: uint::U256::from_big_endian(&hex::decode(p).unwrap()),
            a: a.into(),
            b: b.into(),
            g: (uint::U256::from_big_endian(&hex::decode(g.0).unwrap()),
                uint::U256::from_big_endian(&hex::decode(g.1).unwrap())),
            n: uint::U256::from_big_endian(&hex::decode(n).unwrap()),
            h: uint::U256::from_big_endian(&hex::decode(h).unwrap()),

        }
    }
}

fn main() {
    let secp256k1 = EllipticCurve::new(
        "secp256k1",
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        0,
        7,
        ("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
         "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"),
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        "01");
    println!("Hello, world!");
    let a = uint::U256::from_dec_str("2049").unwrap();
    let b = uint::U256::from_dec_str("2048").unwrap();
    let d = uint::U256::from_big_endian(&hex::decode("ff").unwrap());


    println!("{:?}", a);
    println!("{:?}", a%b);
    println!("{:?}", d);
    println!("{:?}", secp256k1);
}
