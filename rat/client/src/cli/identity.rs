use base64::{Engine as _, engine::general_purpose};

pub fn run() {
    let mut rand_generator = rand::rngs::OsRng;
    let identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let encoded_private_key = general_purpose::STANDARD.encode(identity_keypair.secret.to_bytes());
    println!("private key: {}", encoded_private_key);

    let encoded_public_key = general_purpose::STANDARD.encode(identity_keypair.public.to_bytes());
    println!("public key: {}", encoded_public_key);
}
