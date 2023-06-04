use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::key_exchange::{self, DhKeyPair};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyExchange {
    pub your_public_key: [u8; 32],
    pub your_static_secret: [u8; 32],
    pub other_person_public_key: [u8; 32],
    pub shared_secret: [u8; 32],
    pub encryption_key: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeySignature {
    pub your_public_key: [u8; 32],
    pub your_private_key: [u8; 32],
    pub other_person_public_key: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyStorage {
    pub exchange_map: HashMap<String, (KeyExchange, KeySignature)>,
}

impl KeyStorage {
    pub fn new() -> Self {
        KeyStorage {
            exchange_map: HashMap::new(),
        }
    }

    pub fn create_exchange(
        &mut self,
        exchange_name: &str,
        overwrite: bool,
        key_exchange: Option<KeyExchange>,
        key_signature: Option<KeySignature>,
    ) -> Result<(), &'static str> {
        if self.exchange_map.contains_key(exchange_name) && !overwrite {
            return Err("Key Exchange already exists in Key Storage");
        }

        let mut local_key_exchange = KeyExchange::new();
        let mut local_key_signature = KeySignature::new();

        match (key_exchange, key_signature) {
            (Some(ex), Some(sign)) => {
                local_key_exchange = ex;
                local_key_signature = sign;
            }
            (Some(ex), None) => {
                local_key_exchange = ex;
            }
            (None, Some(_)) => {
                return Err("Invalid constructor with only key signature");
            }
            (None, None) => {}
        }

        _ = self.exchange_map.insert(
            exchange_name.to_string(),
            (local_key_exchange, local_key_signature),
        );
        Ok(())
    }

    pub fn get_map(self) -> HashMap<String, (KeyExchange, KeySignature)> {
        self.exchange_map
    }

    pub fn get_exchange(&self, exchange_name: &str) -> Result<&KeyExchange, &'static str> {
        match self.exchange_map.get(exchange_name) {
            None => Err("Key Exchange doesn't exist in Key Storage"),
            Some((exchange, _)) => Ok(exchange),
        }
    }

    pub fn get_signature(&self, exchange_name: &str) -> Result<&KeySignature, &'static str> {
        match self.exchange_map.get(exchange_name) {
            None => Err("Key Exchange doesn't exist in Key Storage"),
            Some((_, signature)) => Ok(signature),
        }
    }
}

impl KeyExchange {
    pub fn new() -> Self {
        KeyExchange {
            your_public_key: Default::default(),
            your_static_secret: Default::default(),
            other_person_public_key: Default::default(),
            shared_secret: Default::default(),
            encryption_key: Default::default(),
        }
    }

    pub fn get_your_public_key(&self) -> [u8; 32] {
        self.your_public_key
    }

    pub fn add_your_dh_kp(&mut self, kp: DhKeyPair) {
        self.your_public_key = kp.public;
        self.your_static_secret = kp.secret;
    }

    pub fn add_your_public_key(&mut self, pub_k: [u8; 32]) {
        self.your_public_key = pub_k.clone();
    }

    pub fn add_your_static_secret(&mut self, static_secret: [u8; 32]) {
        self.your_static_secret = static_secret.clone();
    }

    pub fn add_other_person_public_key(&mut self, pub_k: [u8; 32]) {
        self.other_person_public_key = pub_k.clone();
    }

    pub fn add_shared_secret(&mut self, shared: [u8; 32], remove_static_secret: bool) {
        self.shared_secret = shared.clone();
        if remove_static_secret {
            self.your_static_secret = Default::default();
        }
    }

    pub fn add_encryption_key(&mut self, encryption_key: [u8; 32], remove_all: bool) {
        self.encryption_key = encryption_key.clone();
        if remove_all {
            self.your_public_key = Default::default();
            self.your_static_secret = Default::default();
            self.other_person_public_key = Default::default();
            self.shared_secret = Default::default();
        }
    }
}

impl KeySignature {
    pub fn new() -> Self {
        KeySignature {
            your_public_key: Default::default(),
            your_private_key: Default::default(),
            other_person_public_key: Default::default(),
        }
    }

    pub fn add_your_public_key(&mut self, pub_k: [u8; 32]) {
        self.your_public_key = pub_k.clone();
    }

    pub fn add_your_private_key(&mut self, priv_k: [u8; 32]) {
        self.your_public_key = priv_k.clone();
    }

    pub fn add_other_person_public_key(&mut self, pub_k: [u8; 32]) {
        self.your_public_key = pub_k.clone();
    }
}
