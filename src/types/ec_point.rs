use crate::types::Point;

#[derive(Debug,PartialEq,Clone,Copy)]
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
