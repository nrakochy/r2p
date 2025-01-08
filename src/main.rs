use r2p::configuration::get_configuration;
use r2p::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration file");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(addr).expect("Failed to bind it");
    run(listener)?.await
}
