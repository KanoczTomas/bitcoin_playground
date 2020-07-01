use crate::types::Point;

#[derive(Debug,PartialEq,Clone,Copy)]
/// Represents a point on curve
pub enum ECpoint {
    Infinity,
    OnCurve(Point),
}

impl std::fmt::LowerHex for ECpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ECpoint::Infinity => write!(f, "Infinity (0)")?,
            ECpoint::OnCurve(p) => write!(f, "({:#x}, {:#x})", p.x, p.y)?
        };
        Ok(())
    }
}

impl ECpoint {
    pub fn to_finite_point(self) -> Point {
        match self {
            ECpoint::Infinity => panic!("ECpoint.to_finite_point() can not convert Infinity!"),
            ECpoint::OnCurve(p) => p
        }
    }
}
