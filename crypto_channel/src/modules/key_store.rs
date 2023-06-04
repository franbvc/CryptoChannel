use std::fs;
use std::path::PathBuf;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

use crate::config::{find_config_file, read_config_file, ConfigSettings};
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
