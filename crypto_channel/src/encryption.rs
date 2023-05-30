use aes_gcm::aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit};
use aes_gcm::Aes256Gcm;
use rand_core::OsRng;

pub fn encrypt_message(message: &[u8], key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, message)
        .expect("Encryption successful");

    (ciphertext, nonce.to_vec())
}

pub fn decrypt_message(encrypted_msg: Vec<u8>, nonce: &Vec<u8>, key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(&nonce);

    cipher
        .decrypt(&nonce, encrypted_msg.as_ref())
        .expect("Decryption successful")
}
