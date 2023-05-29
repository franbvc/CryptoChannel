use base64::{engine::general_purpose, Engine as _};
use rand_core::OsRng;
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
