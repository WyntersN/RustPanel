use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
    pub security_dir: String,
    pub session_ttl: i64,
    pub secret_key: String,
    pub workers: usize,
    pub locale: String,
    pub max_file_size:f64
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub path: String,
    pub max_pool: u32,
}

impl Config {
    pub fn load_from_file() -> Self {
        let mut file = File::open("./config/conf.yaml").expect("conf.yaml...Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("conf.yaml...Failed to read file");
        if contents.contains("workers: 0") {
            contents = contents.replace("workers: 0", &format!("workers: {}",sys_info::cpu_num().expect("sys_info::cpu_num...Failed to get OS release")));
        }
        serde_yaml::from_str(&contents).expect("conf.yaml...Failed to parse YAML")
    }
}
