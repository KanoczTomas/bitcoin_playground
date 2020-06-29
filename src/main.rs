use bitcoin_playground::types::{Points, EllipticCurve};
use bitcoin_playground::types::Errors;
use bitcoin_playground::ec_math::scalar_mult;
use bitcoin_playground::utils::{make_keypair, sign_message, verify_signature};

fn main() -> Result<(), Errors> {
    let secp256k1 = EllipticCurve::secp256k1_factory();
    println!("{:#?}", &secp256k1);
    println!("\n--------------------------------------------------------\n");
    // ECDH
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

    // ECDSA
    println!("");
    #[allow(non_snake_case)]
    let (a, A) = make_keypair(&secp256k1)?;
    println!("Private key: {:#x}", a);
    println!("Public key: {:#x}", A);
    println!("");

    let mut rng = rand::thread_rng();
    let signature = sign_message(&mut rng, a, b"Hello", &secp256k1)?;
    println!("Scenario: Alice signs the message (sig should be ok).");
    println!("Message: '{}'", "Hello");
    println!("Signature: {:#x}", signature);
    let ver = verify_signature(A, b"Hello", &signature, &secp256k1)?;
    println!("Verification: {:?}", ver);
    println!("");

    let second_signature = sign_message(&mut rng, a, b"Hello", &secp256k1)?;
    println!("Scenario: Alice signs the message[again] (sig should be ok, but it is a different pair (r, s)).");
    println!("Message: '{}'", "Hello");
    println!("Signature: {:#x}", second_signature);
    let ver = verify_signature(A, b"Hello", &second_signature, &secp256k1)?;
    println!("Verification: {:?}", ver);
    println!("");

    println!("Scenario: A different message is supplied to verification function (sig should fail[different hash of message]).");
    println!("Message: '{}'", "Hellow world!");
    println!("Signature: {:#x}", signature);
    let ver = verify_signature(A, b"Hello world!", &signature, &secp256k1)?;
    println!("Verification: {:?}", ver);
    println!("");

    println!("Scenario: Bob (not Alice) signs the message (but we are using Alice's pub key to verify, so it will fail).");
    println!("Message: '{}'", "Hello");
    let (b, _) = make_keypair(&secp256k1)?;
    let other_sig = sign_message(&mut rng, b, b"Hello", &secp256k1)?;
    println!("Signature: {:#x}", other_sig);
    let ver = verify_signature(A, b"Hello", &other_sig, &secp256k1)?;
    println!("Verification: {:?}", ver);

    Ok(())
}
