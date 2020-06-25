use crate::types::{Point, ECpoint};


#[derive(Debug, PartialEq,Clone,Copy)]
pub enum Points{
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
