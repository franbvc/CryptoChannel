use std::fs;
use std::path::PathBuf;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

use crate::config::{find_config_file, read_config_file, ConfigSettings};
use crate::key_exchange::key_to_string;
use crate::key_store_classes::{KeyExchange, KeySignature, KeyStorage};

fn create_storage(storage_path: &PathBuf) {
    println!("Creating storage file...");
    _ = fs::File::create(&storage_path).expect("Storage file created!");

    let new_ks = KeyStorage::new();
    let j = serde_json::to_string(&new_ks).unwrap();
    _ = fs::write(storage_path, j).expect("Default storage written successfully!");

    println!("Storage file create at: {}", storage_path.display());
}

fn get_storage_path() -> PathBuf {
    let cfg_path = find_config_file();
    let cfg: ConfigSettings = read_config_file(cfg_path);
    cfg.storage_path
}

pub fn read_storage() -> KeyStorage {
    let storage_path = get_storage_path();

    match fs::read_to_string(&storage_path) {
        Ok(contents) => {
            return serde_json::from_str(&contents).unwrap();
        }
        Err(_) => create_storage(&storage_path),
    }

    let contents = fs::read_to_string(storage_path).expect("Storage file read successfully!");

    serde_json::from_str(&contents).unwrap()
}

pub fn get_key_exchange_names() -> Vec<String> {
    let curr_storage = read_storage();
    curr_storage
        .exchange_map
        .keys()
        .cloned()
        .collect::<Vec<String>>()
}

pub fn get_exchange_dh_public(exchange_name: String) -> String {
    let curr_storage = read_storage();
    let public = curr_storage
        .get_exchange(&exchange_name.to_owned())
        .unwrap()
        .get_your_public_key();
    key_to_string(public)
}

pub fn get_exchange_dh_secret(exchange_name: String) -> [u8; 32] {
    let curr_storage = read_storage();
    curr_storage
        .get_exchange(&exchange_name.to_owned())
        .unwrap()
        .get_your_static_secret()
}

pub fn get_exchange_encryption_key(exchange_name: String) -> [u8; 32] {
    let curr_storage = read_storage();
    curr_storage
        .get_exchange(&exchange_name.to_owned())
        .unwrap()
        .get_encryption_key()
}

pub fn validate_new_exchange_name(exchange_name: String) -> Result<(), &'static str> {
    let curr_storage = read_storage();
    if !curr_storage.name_exists(exchange_name) {
        return Ok(());
    }
    Err("Exchange name already exists in key storage!")
}

pub fn write_storage(to_write: KeyStorage) -> Result<(), &'static str> {
    let storage_path = get_storage_path();
    let j = serde_json::to_string(&to_write).unwrap();

    match fs::write(storage_path, j) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write to Storage"),
    }
}

pub fn test() {
    println!("Finding Storage...");
    println!("Storage Contents: {:?}", read_storage());
}
