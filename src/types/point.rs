use crate::types::u256::U256;

#[derive(Debug,PartialEq,Clone,Copy)]
/// Represents coordinates x, y of a point.
pub struct Point {
    pub x: U256,
    pub y: U256
}

impl Point {
    pub fn new(x: U256, y: U256) -> Self {
        Self {x, y}
    }
}

impl std::convert::From<(U256, U256)> for Point {
    fn from(t: (U256, U256)) -> Self {
        Point::new(t.0, t.1)
    }
}
