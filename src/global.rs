use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub static RT: OnceLock<Runtime> = OnceLock::new();

pub fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().expect("Create Tokio runtime failed"))
}
