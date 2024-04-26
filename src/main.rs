#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    r2p::run().await
}
