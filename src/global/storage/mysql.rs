use super::super::error::app::AppError;
use crate::conf;
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
use tokio::runtime::Runtime;

pub fn init_mysql_pool(rt: &Runtime) -> Result<Pool<MySql>, AppError> {
    let mysql_conf = conf::mysql_config();
    let dsn = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_conf.user, mysql_conf.password, mysql_conf.host, mysql_conf.port, mysql_conf.database
    );

    let pool = rt.block_on(async {
        MySqlPoolOptions::new()
            .max_connections(mysql_conf.max_conn)
            .connect(&dsn)
            .await
    })?;

    Ok(pool)
}
