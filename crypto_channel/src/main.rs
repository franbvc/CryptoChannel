mod key_exchange;
use key_exchange::{key_to_string, string_to_key};

fn main() {
    let (_, _) = key_exchange::gen_ephemeral_kp();
    let (a_priv_k, a_pub_k) = key_exchange::gen_static_kp();
    let (b_priv_k, b_pub_k) = key_exchange::gen_static_kp();

    //let encoded_pub = key_to_string(pub_k);

    //println!("Encoded: {:?}", encoded_pub);
    ////println!("{:?}", priv_k.as_bytes());
    //println!("Byte Array: {:?}", pub_k.as_bytes());

    //let decoded_pub = string_to_key(encoded_pub);
    //println!("Decoded: {:?}", decoded_pub);

    //assert!(pub_k.as_bytes().to_vec() == decoded_pub);

    let a_shared = key_exchange::gen_shared_secret(b_pub_k, a_priv_k);
    let b_shared = key_exchange::gen_shared_secret(a_pub_k, b_priv_k);

    println!("a: {:?}", a_shared.as_bytes());
    println!("b: {:?}", b_shared.as_bytes());

    assert!(a_shared.as_bytes() == b_shared.as_bytes());

    println!("Hello, world!");
}
