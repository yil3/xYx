use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    pub log: LogConfig,
    pub auth: AuthConfig,
    pub user_resources_server: Option<UserResourceServer>,
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
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<u32>,
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
    pub token_expired: Option<u64>,
    pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserResourceServer {
    pub url: String,
}

impl AppConfig {
    pub fn parse() -> Self {
        let path1 = std::env::current_exe()
            .as_ref()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .unwrap_or_default()
            .to_owned()
            + "/application.yml";
        let yml_str = match read_to_string(&path1) {
            Ok(str) => str,
            Err(_) => {
                let path2 = std::fs::read_dir("./")
                    .unwrap()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_name().to_str().unwrap().contains("application"))
                    .map(|entry| entry.file_name().to_str().unwrap().to_owned())
                    .collect::<Vec<String>>()[0]
                    .clone();
                match read_to_string(&path2) {
                    Ok(str) => str,
                    Err(_) => {
                        panic!("application.yml not found");
                    },
                }
            },
        };
        serde_yaml::from_str(&yml_str).unwrap()
    }
}
