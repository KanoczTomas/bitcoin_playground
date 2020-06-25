use crate::types::u256::U256;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Point {
    pub x: U256,
    pub y: U256
}

impl Point {
    pub fn new(_x: U256, _y: U256) -> Self {
        Self {
            x: _x,
            y: _y
        }
    }
}

impl std::convert::From<(U256, U256)> for Point {
    fn from(t: (U256, U256)) -> Self {
        Point::new(t.0, t.1)
    }
}
