use learn::{conf, global, routers};
use learn::utils::error::{
    base::GlobalAppError as Error
};

fn main() {
    global::rt().block_on(async {
        if let Err(e) = run_server().await {
            eprintln!("服务启动错误： {:?}", e)
        }
    });
}

async fn run_server() -> Result<(), Error> {
    // 初始化日志输出
    tracing_subscriber::fmt()
        // .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .init();

    let router = routers::create_router();
    let server_config = conf::server_config();
    let listener = tokio::net::TcpListener::bind(&server_config.addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
