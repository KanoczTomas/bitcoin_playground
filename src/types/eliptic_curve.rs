use crate::types::{U256, Point, ECpoint};

#[derive(Debug)]
pub struct EllipticCurve {
    ///name of the curve
    pub name: String,
    ///prime (modulo)
    pub p: U256,
    ///a coefficient
    pub a: i64,
    ///b coefficient
    pub b: i64,
    ///base point
    pub g: (U256, U256),
    ///subgroup order
    pub n: U256,
    ///subgroup cofactor
    pub h: U256
}


impl EllipticCurve {
    // if a str is given with 0x it will treat it as hexa string otherwise a decadic number
    fn pick_hex_or_dec(s: &str) -> U256{
        match s.starts_with("0x"){
            true => U256::from_big_endian(&hex::decode(s.trim_start_matches("0x")).unwrap()),
            false => U256::from_dec_str(s).unwrap()
        }
    }
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            p: U256::default(),
            a: i64::default(),
            b: i64::default(),
            g: (U256::default(),
                U256::default()),
            n: U256::default(),
            h: U256::default(),
        }
    }
    fn set_p(&mut self, _p: &str) -> &mut Self{
        self.p = Self::pick_hex_or_dec(_p);
        self
    }
    fn set_a(&mut self, _a: i64) -> &mut Self{
        self.a = _a;
        self
    }
    fn set_b(&mut self, _b: i64) -> &mut Self{
        self.b = _b;
        self
    }
    fn set_g(&mut self, _g: (&str, &str)) -> &mut Self{
        self.g = (Self::pick_hex_or_dec(_g.0), Self::pick_hex_or_dec(_g.1));
        self
    }
    fn set_n(&mut self, _n: &str) -> &mut Self{
        self.n = Self::pick_hex_or_dec(_n);
        self
    }
    fn set_h(&mut self, _h: &str) -> &mut Self{
        self.h = Self::pick_hex_or_dec(_h);
        self
    }
    ///Constructs secp256k1 EllipticCurve
    pub fn secp256k1_factory() -> EllipticCurve {
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
}

impl std::fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "EllipticCurve {{")?;
        writeln!(f, "\tname: {}", self.name)?;
        writeln!(f, "\tp: {:#x}", self.p)?;
        writeln!(f, "\ta: {:#x}", self.a)?;
        writeln!(f, "\tb: {:#x}", self.b)?;
        writeln!(f, "\tg: {:#x}", ECpoint::OnCurve(Point::from(self.g)))?;
        writeln!(f, "\tn: {:#x}", self.n)?;
        writeln!(f, "\th: {:#x}", self.h)?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
