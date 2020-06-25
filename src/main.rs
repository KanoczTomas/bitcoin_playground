use bitcoin_playground::types::{Points, EllipticCurve};
use bitcoin_playground::types::Errors;
use bitcoin_playground::ec_math::scalar_mult;
use bitcoin_playground::utils::make_keypair;

fn main() -> Result<(), Errors> {
    let secp256k1 = EllipticCurve::secp256k1_factory();
    println!("{}", &secp256k1);
    println!("\n--------------------------------------------------------\n");
    println!("ECDH data: Alice priv key(a), pub key(A), Bob priv key(b), pub key(B)");
    #[allow(non_snake_case)]
    let (a, A) = make_keypair(&secp256k1)?;
    #[allow(non_snake_case)]
    let (b, B) = make_keypair(&secp256k1)?;
    println!("Alice priv key(a): {:#x}", a);
    println!("Alice pub key(A): {:#x}", A);
    println!("");
    println!("Bob priv key(b): {:#x}", b);
    println!("Bob pub key(B): {:#x}", B);
    println!("");
    println!("Shared key for Alice => a * B");
    #[allow(non_snake_case)]
    let B: Points = B.into();
    println!("==> {:#x}", scalar_mult(a, &B, &secp256k1)?);
    println!("Shared key for Bob => b * A");
    #[allow(non_snake_case)]
    let A: Points = A.into();
    println!("==> {:#x}", scalar_mult(b, &A, &secp256k1)?);
    Ok(())
}
