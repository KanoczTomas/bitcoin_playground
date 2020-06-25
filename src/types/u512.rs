use uint::construct_uint;
use crate::types::U256;
use std;

construct_uint! {
    pub struct U512(8);
}

impl std::convert::From<U256> for U512 {
    fn from(n: U256) -> Self {
        let U256(ref arr) = n;
        let mut ret = [0u64;8];
        ret[0]= arr[0];
        ret[1]= arr[1];
        ret[2]= arr[2];
        ret[3]= arr[3];
        U512(ret)
    }
}
