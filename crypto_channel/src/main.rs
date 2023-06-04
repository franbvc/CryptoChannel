use modules::config;
use modules::encryption;
use modules::key_exchange;
use modules::key_store;
use modules::key_store_classes;

use arboard::Clipboard;
use key_exchange::{key_to_string, string_to_key};

use crate::key_exchange::gen_encryption_key;

fn main() {
    //test();
    config::test();
    key_store::test();

    //let (_, _) = key_exchange::gen_ephemeral_kp();
    //let (a_priv_k, a_pub_k) = key_exchange::gen_static_kp();
    //let (b_priv_k, b_pub_k) = key_exchange::gen_static_kp();

    //let encoded_pub = key_to_string(pub_k);

    //println!("Encoded: {:?}", encoded_pub);
    ////println!("{:?}", priv_k.as_bytes());
    //println!("Byte Array: {:?}", pub_k.as_bytes());

    //let decoded_pub = string_to_key(encoded_pub);
    //println!("Decoded: {:?}", decoded_pub);

    //assert!(pub_k.as_bytes().to_vec() == decoded_pub);

    //let a_shared = key_exchange::gen_shared_secret(b_pub_k, a_priv_k);
    //let b_shared = key_exchange::gen_shared_secret(a_pub_k, b_priv_k);

    //println!("a: {:?}", a_shared.as_bytes());
    //println!("b: {:?}", b_shared.as_bytes());

    //assert!(a_shared.as_bytes() == b_shared.as_bytes());

    //let key = key_exchange::gen_encryption_key(a_shared);
    //let message = "This is a test!";

    //println!("key: {:?}", key);

    //let (encrypted_msg, nonce) = encryption::encrypt_message(message.as_bytes(), &key);

    //println!("encrypted: {:?}", encrypted_msg);
    //println!("nonce: {:?}", nonce);

    //let decrypted_msg = encryption::decrypt_message(encrypted_msg, &nonce, &key);

    //println!("decrypted: {:?}", String::from_utf8(decrypted_msg));
    //println!("Hello, world!");

    //let mut clipboard = Clipboard::new().unwrap();
    //println!("Clipboard text was: {}", clipboard.get_text().unwrap());

    //let the_string = "Hello, world!";
    //clipboard.set_text(the_string).unwrap();
    //println!("But now the clipboard text should be: \"{}\"", the_string);
}
