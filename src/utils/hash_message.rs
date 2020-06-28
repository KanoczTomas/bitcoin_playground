use crate::types::{U256, U512, EllipticCurve};
use ring::digest::{digest, SHA512};

///Returns the truncated SHA512 hash of the message.
pub fn hash_message(message: &[u8], curve: &EllipticCurve) -> U256 {
    let digest = digest(&SHA512, message);
    let digest = U512::from(digest.as_ref());
    //FIPS 180 says that when a hash needs to be truncated, the rightmost bits
    // should be discarded.
    //https://security.stackexchange.com/questions/72673/how-bad-is-it-to-truncate-a-hash#72675
    let digest = digest >> (digest.bits() - curve.n.bits());
    U256::from(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_message() {
        let secp256k1 = EllipticCurve::secp256k1_factory();
        assert_eq!(format!("{:x}",hash_message(b"This is a test string", &secp256k1)), "b8ee69b29956b0b56e26d0a25c6a80713c858cf2902a12962aad08d682345646");
        assert_eq!(format!("{:x}",hash_message(b"This is a test string\n", &secp256k1)), "ab6ddc5c40d0ed2fcdbf00c71ff80811c2b6eb274dccce690a50a6f7595fca2a");
        assert_eq!(format!("{:x}",hash_message(b".", &secp256k1)), "b61241d7c17bcbb1baee7094d14b7c451efecc7ffcbd92598a0f13d313cc9ebc");
    }
}
