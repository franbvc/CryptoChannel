use base64::{engine::general_purpose, Engine as _};
use hex_literal::hex;
use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret, StaticSecret};

pub fn gen_ephemeral_kp() -> (EphemeralSecret, PublicKey) {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

pub fn gen_static_kp() -> (StaticSecret, PublicKey) {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

pub fn key_to_string(pub_k: PublicKey) -> String {
    general_purpose::STANDARD.encode(pub_k.as_bytes())
}

pub fn string_to_key(key_str: String) -> Vec<u8> {
    general_purpose::STANDARD.decode(key_str).unwrap()
}

pub fn gen_shared_secret(pub_k: PublicKey, priv_k: StaticSecret) -> SharedSecret {
    priv_k.diffie_hellman(&pub_k)
}

pub fn gen_encryption_key(priv_k: SharedSecret) -> [u8; 32] {
    let info = hex!("f0f1f2f3f4f5f6f7f8f9");
    let prk = priv_k.as_bytes();
    let hk = Hkdf::<Sha256>::from_prk(prk).expect("PRK should be large enough");
    let mut okm = [0u8; 32];
    hk.expand(&info, &mut okm)
        .expect("32 is a valid length for Sha256 to output");
    okm
}
