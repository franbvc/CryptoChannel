use modules::key_exchange::{gen_static_kp, DhKeyPair};
use modules::key_store::{self, write_storage};
use modules::key_store_classes::{self, KeyExchange};

#[test]
fn print_storage() {
    let curr_storage = key_store::read_storage();
    println!("{:?}", curr_storage);
}

#[test]
fn save_your_dh_pair() {
    let alice_dh_kp = gen_static_kp();
    let mut curr_storage = key_store::read_storage();

    let mut new_exchange = KeyExchange::new();
    new_exchange.add_your_dh_kp(alice_dh_kp);

    println!("Exchange public: {:?}", new_exchange.your_public_key);
    println!("Exchange secret: {:?}", new_exchange.your_static_secret);

    _ = curr_storage.create_exchange("Bob", false, Some(new_exchange), None);

    println!("{:?}", curr_storage.exchange_map);

    assert!(write_storage(curr_storage).is_ok());
}
