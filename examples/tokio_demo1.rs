#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async { "return value" });
    let out = handle.await.unwrap();
    println!("{}", out);
}
