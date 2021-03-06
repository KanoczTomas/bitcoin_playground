use crate::types::U256;

/// Represents an EC signature.
#[derive(Debug, PartialEq)]
pub struct Signature {
    pub r: U256,
    pub s: U256
}

impl Signature {
    /// Creates instance of Signature from r, s.
    pub fn new(r: U256, s: U256) -> Self {
        Signature {r, s}
    }
}

impl std::convert::From<(U256, U256)> for Signature {
    fn from(tuple: (U256, U256)) -> Self {
        Signature::new(tuple.0, tuple.1)
    }
}

impl std::fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let Signature {r, s} = self;
        write!(f, "({:#x}, {:#x})", r, s)?;
        Ok(())
    }
}
