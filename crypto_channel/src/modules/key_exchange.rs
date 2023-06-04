use base64::{engine::general_purpose, Engine as _};
use hex_literal::hex;
use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret, StaticSecret};

pub struct DhKeyPair {
    pub public: [u8; 32],
    pub secret: [u8; 32],
}

pub fn gen_ephemeral_kp() -> (EphemeralSecret, PublicKey) {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

pub fn gen_static_kp() -> DhKeyPair {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    DhKeyPair {
        public: *public.as_bytes(),
        secret: *secret.as_bytes(),
    }
}

pub fn key_to_string(key: [u8; 32]) -> String {
    general_purpose::STANDARD.encode(key)
}

pub fn string_to_key(key_str: String) -> [u8; 32] {
    let key_vec = general_purpose::STANDARD.decode(key_str).unwrap();

    if key_vec.len() != 32 {
        panic!("Key Vector has not 32 elements!");
    }

    let mut key_arr = [0u8; 32];
    for i in 0..32 {
        key_arr[i] = key_vec[i];
    }

    key_arr
}

pub fn gen_shared_secret(pub_k: [u8; 32], priv_k: [u8; 32]) -> [u8; 32] {
    let public = PublicKey::from(pub_k);
    let secret = StaticSecret::from(priv_k);

    *secret.diffie_hellman(&public).as_bytes()
}

pub fn gen_encryption_key(prk: [u8; 32]) -> [u8; 32] {
    let info = hex!("f0f1f2f3f4f5f6f7f8f9");
    let hk = Hkdf::<Sha256>::from_prk(&prk).expect("PRK should be large enough");
    let mut okm = [0u8; 32];
    hk.expand(&info, &mut okm)
        .expect("32 is a valid length for Sha256 to output");
    okm
}
