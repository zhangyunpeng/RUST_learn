use std::sync::OnceLock;
use serde::Deserialize;
use config::{Config, File};

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn config() -> &'static AppConfig {
    CONFIG.get_or_init(||load_config())
}

pub fn server_config() -> &'static ServerConfig {
    CONFIG.get_or_init(||load_config());
    &CONFIG.get().unwrap().server
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub mysql: MysqlConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub shutdown_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub max_conn: u32,
}

/// 加载项目根目录 app.yaml
pub fn load_config() -> AppConfig {
    let cfg =  Config::builder()
        .add_source(File::with_name("app"))
        .build()
        .expect("app.yaml 不存在，请放在项目根目录");
    // 去掉末尾 ;，表达式自动作为函数返回值
    cfg.try_deserialize().expect("app.yaml 格式错误或字段缺失")
}

