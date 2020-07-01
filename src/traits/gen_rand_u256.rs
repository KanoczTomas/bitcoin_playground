use crate::types::U256;
use rand::Rng;
//inspired by https://github.com/rust-num/num-bigint/blob/master/src/bigrand.rs

/// Trait to generate random U256 numbers
pub trait GenRandU256 {
    /// Generate a random U256.
    fn gen_u256(&mut self) -> U256;

    /// Generate a random `U256` within the given range. The lower
    /// bound is inclusive; the upper bound is exclusive. Fails when
    /// the upper bound is not greater than the lower bound.
    fn gen_u256_range(&mut self, lbound: &U256, ubound: &U256) -> U256;
}

impl<R: Rng + ?Sized> GenRandU256 for R {
    fn gen_u256(&mut self) -> U256 {
        let mut data = [0u64; 4];
        self.fill(&mut data);
        U256(data)
    }
    fn gen_u256_range(&mut self, low: &U256, high: &U256) -> U256 {
        let mut num;
        loop {
            num = self.gen_u256();
            if num >= *low && num < *high {
                break;
            }
        }
        num
    }
}
