# bitcoin_playground
playing with ECDSA, ECDH, EC math to understand bitcoin in rust

## Inspiration
This little project was inspired by a wonderful guide through Elliptic curve math:

https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/.

The tutorial series includes a lot of material to help understand EC math. Best way to learn is to do things, I wanted to implement the scripts my self in python,
then I realised it can be a great way to learn Rust and EC at the same time.

So here it is, my effort to understand EC math and learn Rust.

## Setup
1. clone this repo
1. run tests with `cargo run test --release` (release is much faster, as is optimized)
1. run the main function: `cargo run --release`

The scripts are only for playing around, do not use it in production code. I did implement a trait to generate random unsigned 256 bit integers, but am not sure
how good source of randomness it is. Please do not use it for production, just for learning.

Sample output:

```rust
EllipticCurve {
    name: "secp256k1",
    p: 115792089237316195423570985008687907853269984665640564039457584007908834671663,
    a: 0,
    b: 7,
    g: (
        55066263022277343669578718895168534326250603453777594175500187360389116729240,
        32670510020758816978083085130507043184471273380659243275938904335757337482424,
    ),
    n: 115792089237316195423570985008687907852837564279074904382605163141518161494337,
    h: 1,
}

--------------------------------------------------------

ECDH data: Alice priv key(a), pub key(A), Bob priv key(b), pub key(B)
Alice priv key(a): 0x7094e3e71de69e191d249ad3f11ef351f42c455553406d264821894a1fece00d
Alice pub key(A): (0x1e28a18615de725d9d582728ef638812d42e6e2c75c8ca94237a6ccfdba155f4, 0x6131a45944f63910fdd71f48687124b871271a8c4cb28805f97b2b6790fb01d)

Bob priv key(b): 0x2712ba070242502a84cd61aa0a7e328ced9cb58a44b88fd8021a33e2630e3d69
Bob pub key(B): (0x7edce63ac7fa3c0abae8e7b30bc3a9755873b5250e966c184288f6c7d37920f0, 0x4865bf4c43dd06e812eca8e11cd289d8171b248865916944554f73931a6fc485)

Shared key for Alice => a * B
==> (0xd6d27c19022c8d25d86ad349c55af955d8cb7542b5a11034e82ad8b344d78a94, 0x3e742ae04c538561296871f61cb5088890b184ebb6da173c24e1605616691751)
Shared key for Bob => b * A
==> (0xd6d27c19022c8d25d86ad349c55af955d8cb7542b5a11034e82ad8b344d78a94, 0x3e742ae04c538561296871f61cb5088890b184ebb6da173c24e1605616691751)

ECDSA

Private key(Alice): 0x185409a4af6b23d7f7874862aa81a84b75883cb73f109f7bdc6f6c8130b0936
Public key(Alice): (0xbd103eb3bc2e9a910f5c6282c973809fba51ac12921464fbb6d762bc1f87589f, 0x2592e893c426a381457a719bea0e06d80b7ca46709f4efb21040f575ec18a847)

Scenario: Alice signs the message (sig should be ok).
Message: 'Hello'
Signature: (0x3325ad2caf86cad11c7d777e92e1352cf63e29d176d50faddcf57b496c8418ab, 0xa34021aa5cdfe16fcaef661dec86d6d68f2a38722e7d82344f0eb65f77c44a97)
Verification: Successful

Scenario: Alice signs the message[again] (sig should be ok, but it is a different pair (r, s)).
Message: 'Hello'
Signature: (0x536eb26ddd6b23171229a7bb9d7db0ecd3f65f0e01b9813ac4cf6f11284e8f81, 0xba5b405786f3581bb6530790553ff4033c163a6fe1e06d250e00dec107ec226c)
Verification: Successful

Scenario: A different message is supplied to verification function (sig should fail[different hash of message]).
Message: 'Hellow world!'
Signature: (0x3325ad2caf86cad11c7d777e92e1352cf63e29d176d50faddcf57b496c8418ab, 0xa34021aa5cdfe16fcaef661dec86d6d68f2a38722e7d82344f0eb65f77c44a97)
Verification: Failed

Scenario: Bob (not Alice) signs the message (but we are using Alice's pub key to verify, so it will fail).
Message: 'Hello'
Signature: (0x50496d53155288e799fc8edce0896f6c95e9f01539c9332e1fd5e4b226b33607, 0x55de6207fb491129e853df61145d7e7d854aecd57d51f5089e6803c9fa0e820f)
Verification: Failed

```
