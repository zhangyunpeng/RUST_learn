use crate::global::storage;
use sqlx::MySqlPool;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub static RT: OnceLock<Runtime> = OnceLock::new();

pub fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().expect("Create Tokio runtime failed"))
}

pub static MYSQL_POOL: OnceLock<MySqlPool> = OnceLock::new();

pub fn init_mysql_pool() {
    let pool = storage::mysql::init_mysql_pool(rt()).expect("MySQL连接池初始化失败");
    MYSQL_POOL.set(pool).unwrap();
}

pub fn mysql_pool() -> &'static MySqlPool {
    MYSQL_POOL
        .get_or_init(|| storage::mysql::init_mysql_pool(rt()).expect("Init MySQL pool failed"))
}
