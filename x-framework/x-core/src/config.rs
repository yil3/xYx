use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    pub log: LogConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub password: String,
    pub username: String,
    pub run_migrations: bool,
    pub category: String,
    pub host: String,
    pub port: u32,
    pub db_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u32,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub cors: CorsConfig,
    pub http_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub origins: Vec<String>,
    pub status: bool,
}


#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub argon_salt: String,
    pub token_secret: String,
    pub status: bool,
    pub expired: Option<u64>,
}

impl AppConfig {
    pub fn parse() -> Self {
        // let path = std::env::current_exe()
        //     .unwrap()
        //     .to_str()
        //     .unwrap()
        //     .split("/")
        //     .last()
        //     .unwrap()
        //     .to_owned()
        //     + "/application.yml";
        let filename = std::fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_str().unwrap().contains("application"))
            .map(|entry| entry.file_name().to_str().unwrap().to_owned())
            .collect::<Vec<String>>()[0]
            .clone();
        let yml_str = read_to_string(filename)
            .map_err(|e| anyhow::anyhow!("Failed to read application.yml: {e}"))
            .unwrap();
        serde_yaml::from_str(&yml_str).unwrap()
    }
}
