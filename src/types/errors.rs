use crate::types::{U256, Point};

#[derive(Debug,PartialEq)]
/// Represents errors
pub enum Errors {
    /// Zero division error
    ZeroDivision,
    /// The number is not a multiplicative inverse mod p
    NoMultiplicativeInverse(U256,U256),
    /// Zero modulo error
    ZeroModulo,
    /// Point not on curve
    PointNotOnCurve(Point),
    /// Negative point not on curve(only happens if point_neg is buggy)
    NegativePointNotOnCurve(Point),
}
