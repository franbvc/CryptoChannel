use aes_gcm::aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit};
use aes_gcm::Aes256Gcm;
use base64::{engine::general_purpose, Engine as _};
use rand_core::OsRng;

pub fn encrypt_message(message: &[u8], key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, message)
        .expect("Encryption successful");

    (ciphertext, nonce.to_vec())
}

pub fn encrypted_to_str(ciphertext: Vec<u8>, nonce: Vec<u8>) -> String {
    let ciphertext_str = general_purpose::STANDARD.encode(ciphertext);
    let nonce_str = general_purpose::STANDARD.encode(nonce);
    nonce_str + ";" + &ciphertext_str
}

pub fn str_to_encrypted(s: String) -> (Vec<u8>, Vec<u8>) {
    let split_index = s.find(";").expect("Nonce division found");
    let nonce = String::from(&s[0..split_index]);
    let msg = String::from(&s[split_index + 1..]);

    let nonce_vec = general_purpose::STANDARD.decode(nonce).unwrap();
    let msg_vec = general_purpose::STANDARD.decode(msg).unwrap();

    (msg_vec, nonce_vec)
}

pub fn decrypt_message(encrypted_msg: Vec<u8>, nonce: &Vec<u8>, key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(&nonce);

    cipher
        .decrypt(&nonce, encrypted_msg.as_ref())
        .expect("Decryption successful")
}
