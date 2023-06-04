use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSettings {
    pub storage_path: PathBuf,
    pub encrypt_storage: bool,
    pub use_signature: bool,
}

impl ConfigSettings {
    fn new(home_path: PathBuf) -> Self {
        ConfigSettings {
            storage_path: home_path.join(".config/crypto-channel/storage.json"),
            encrypt_storage: false,
            use_signature: false,
        }
    }
}

pub fn find_config_file() -> PathBuf {
    let home_path = home_dir().unwrap();
    let config_dir_path = home_path.join(".config/crypto-channel");

    let config_dir = fs::read_dir(&config_dir_path);
    match config_dir {
        Ok(_) => println!("Config directory found at: {}", &config_dir_path.display()),
        Err(_) => create_config_dir(&config_dir_path),
    }

    let config_file_path = config_dir_path.join("config.json");

    let config_file = fs::read(&config_file_path);
    match config_file {
        Ok(_) => println!("Config file found at: {}", &config_file_path.display()),
        Err(_) => {
            create_config_file(&config_file_path);
            write_default_config(&config_file_path);
        }
    }

    config_file_path
}

pub fn read_config_file(config_file_path: PathBuf) -> ConfigSettings {
    let contents =
        fs::read_to_string(config_file_path).expect("Should have been able to read the file");

    serde_json::from_str(&contents).unwrap()
}

fn create_config_dir(config_dir_path: &PathBuf) {
    println!("Creating config dir...");
    _ = fs::create_dir(&config_dir_path).expect("Config directory created!");
    println!("Config directory crated at: {}", config_dir_path.display());
}

fn create_config_file(config_file_path: &PathBuf) {
    println!("Creating config file...");
    _ = fs::File::create(&config_file_path).expect("Config file created!");
    println!("Config file created at: {}", config_file_path.display());
}

pub fn write_default_config(config_file_path: &PathBuf) {
    let home_path = home_dir().unwrap();
    let new_config = ConfigSettings::new(home_path);
    let j = serde_json::to_string(&new_config).unwrap();
    _ = fs::write(config_file_path, j).expect("Default settings written successfully!");

    println!("Default settings written to config file!");
}

pub fn test() {
    let cfg_path = find_config_file();
    let cfg = read_config_file(cfg_path);

    println!("{:?}", cfg);
    println!("{}", cfg.storage_path.display());
}
