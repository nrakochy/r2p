use r2p::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("http://127.0.0.1:8000").expect("Failed to bind it");
    run(listener)?.await
}
