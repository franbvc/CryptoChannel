use modules::encryption::{decrypt_message, encrypt_message, encrypted_to_str, str_to_encrypted};
use modules::key_exchange::{gen_encryption_key, gen_shared_secret, gen_static_kp, DhKeyPair};

#[test]
fn test_encryption() {
    let alice_kp = gen_static_kp();
    let bob_kp = gen_static_kp();

    let alice_shared = gen_shared_secret(bob_kp.public, alice_kp.secret);
    let bob_shared = gen_shared_secret(alice_kp.public, bob_kp.secret);

    assert_eq!(alice_shared, bob_shared);

    let alice_encrypt_key = gen_encryption_key(alice_shared);
    let bob_encrypt_key = gen_encryption_key(bob_shared);

    assert_eq!(alice_encrypt_key, bob_encrypt_key);

    let msg_from_alice = "This is a new test";

    let (encrypted_msg_from_alice, alice_nonce) =
        encrypt_message(msg_from_alice.as_bytes(), &alice_encrypt_key);

    let encoded_msg_from_alice =
        encrypted_to_str(encrypted_msg_from_alice.clone(), alice_nonce.clone());

    let (decoded_msg_by_bob, decoded_alice_nonce) = str_to_encrypted(encoded_msg_from_alice);

    assert_eq!(encrypted_msg_from_alice, decoded_msg_by_bob);
    assert_eq!(alice_nonce, decoded_alice_nonce);

    let decrypted_msg_by_bob =
        decrypt_message(decoded_msg_by_bob, &decoded_alice_nonce, &bob_encrypt_key);

    assert_eq!(decrypted_msg_by_bob, msg_from_alice.as_bytes().to_owned());

    println!("{}", std::str::from_utf8(&decrypted_msg_by_bob).unwrap());
}
