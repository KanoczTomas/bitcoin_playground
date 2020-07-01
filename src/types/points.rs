use crate::types::{Point, ECpoint};


#[derive(Debug, PartialEq,Clone,Copy)]
/// Represents all possible points (even those not on curve!).
pub enum Points{
    /// Infinity.
    Infinity,
    /// A point with a coordinate.
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

impl std::convert::From<Point> for Points {
    fn from(p: Point) -> Self {
        Points::FinitePoint(p)
    }
}
