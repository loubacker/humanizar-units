#![forbid(unsafe_code)]

#[tokio::main]
async fn main() {
    if let Err(error) = humanizar_units::infrastructure::server::run().await {
        eprintln!("Falha ao iniciar humanizar-units: {error}");
        std::process::exit(1);
    }
}
