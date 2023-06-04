use modules::key_exchange::{
    gen_encryption_key, gen_shared_secret, gen_static_kp, key_to_string, string_to_key,
};

#[test]
fn test_gen_dh_kp() {
    let kp = gen_static_kp();
    println!("Public: {:?}", kp.public);
    println!("Secret: {:?}", kp.secret);
}

#[test]
fn test_kp_str_conversion() {
    let kp = gen_static_kp();

    let public_str = key_to_string(kp.public);
    let secret_str = key_to_string(kp.secret);

    let public_arr = string_to_key(public_str.clone());
    let secret_arr = string_to_key(secret_str.clone());

    println!("Public key: {:?}", kp.public);
    println!("Public str: {}", public_str);

    println!("Secret key: {:?}", kp.secret);
    println!("Secret str: {}", secret_str);

    assert!(kp.public == public_arr);
    assert!(kp.secret == secret_arr);
}

#[test]
fn test_gen_dh_shared() {
    let alice_kp = gen_static_kp();
    let bob_kp = gen_static_kp();

    let alice_shared = gen_shared_secret(bob_kp.public, alice_kp.secret);
    let bob_shared = gen_shared_secret(alice_kp.public, bob_kp.secret);

    assert!(alice_shared == bob_shared);
}

#[test]
fn test_gen_encrypt_key() {
    let alice_kp = gen_static_kp();
    let bob_kp = gen_static_kp();

    let alice_shared = gen_shared_secret(bob_kp.public, alice_kp.secret);
    let bob_shared = gen_shared_secret(alice_kp.public, bob_kp.secret);

    assert!(alice_shared == bob_shared);

    let alice_encrypt_key = gen_encryption_key(alice_shared);
    let bob_encrypt_key = gen_encryption_key(bob_shared);

    assert!(alice_encrypt_key == bob_encrypt_key);
}
