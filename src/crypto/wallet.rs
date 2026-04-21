use rand::rngs::OsRng;
use ed25519_dalek::{Keypair};

fn generate_wallet() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}