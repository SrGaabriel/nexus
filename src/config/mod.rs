use std::{fs::{self, File}, path::Path, process::exit, time::Instant};

use chrono::Utc;
use serde::{Serialize, Deserialize};

const CONFIG_PATH: &'static str = "/home/sleeper/config.toml";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub secret: String,
    pub database: Database
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: String::from("127.0.0.1"),
            port: 8080,
            secret: Utc::now().timestamp_nanos().to_string(),
            database: Database {
                host: String::from("127.0.0.1"),
                port: 5432,
                database: String::from("sleeper"),
                username: String::from("postgres"),
                password: String::from("postgres")
            }
        }
    }
}

impl Database {
    pub fn assemble(&self) -> String {
        format!("postgresql://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
    }
}

pub fn load() -> Config {
    let config_path = Path::new(CONFIG_PATH);
    if !config_path.exists() {
        log::error!("config file not found, creating one and aborting process");
        File::create(config_path).expect("failed to create config file");
        fs::write(config_path, toml::to_string_pretty(&Config::default()).expect("error while trying to create new config")).expect("error while trying to generate default config specs");
        exit(-1)
    }
    let config: Config = toml::from_str(fs::read_to_string(config_path).expect("error while reading the config file").as_str()).expect("failed to parse config");
    config
}