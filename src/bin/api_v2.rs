use learn::global::error::base::GlobalAppError as Error;
use learn::{conf, instance, routers};

fn main() {
    // 1. 加载配置
    conf::load_config();

    // 2. 初始化全局Runtime（OnceLock保证只创建一次）
    instance::rt();

    // 3. 【同步主线程初始化MySQL】此时还没进入block_on，允许使用rt.block_on
    instance::init_mysql_pool();

    // 4. 启动异步服务，全程只会读取已就绪的连接池，不会再执行初始化逻辑
    instance::rt().block_on(async {
        if let Err(e) = run_server().await {
            eprintln!("服务启动错误： {:?}", e)
        }
    });
}

async fn run_server() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .init();

    let router = routers::create_router();
    let server_config = conf::server_config();
    let listener = tokio::net::TcpListener::bind(&server_config.addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
