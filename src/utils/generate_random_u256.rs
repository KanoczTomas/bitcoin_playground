use crate::types::U256;
use rand;

pub fn generate_random_u256() -> U256 {
    let mut bytes = [0u64; 4];
    for byte in bytes.iter_mut() {
        *byte = rand::random::<u64>();
    }
    U256(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_u256() {
        let num = generate_random_u256();
        match num {
            U256(_) => {},
            #[allow(unreachable_patterns)]
            _ => panic!("return value should be U256")
        }
    }
}
