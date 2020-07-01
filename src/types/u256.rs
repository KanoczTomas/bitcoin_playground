use uint::construct_uint;
use crate::types::U512;

construct_uint! {
    /// A 256 bit little endian unsigned integer.
    pub struct U256(4);
}

impl U256 {
    /// Multiply self with b, not loosing precision as math is done on 512 bits.
    pub fn full_mul(self, b: Self) -> U512 {
        let a = U512::from(self);
        let b = U512::from(b);
        a * b
    }
}

impl std::convert::From<U512> for U256 {
    fn from(n: U512) -> Self {
        let U512(ref arr) = n;
        if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
            panic!("Can not convert U512 to U256, overflow!");
        }
        let mut ret = [0u64;4];
        ret[0]= arr[0];
        ret[1]= arr[1];
        ret[2]= arr[2];
        ret[3]= arr[3];
        U256(ret)
    }
}
