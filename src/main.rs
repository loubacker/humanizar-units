#![forbid(unsafe_code)]

use humanizar_units::infrastructure::diagnostics::StartupReport;
use humanizar_units::infrastructure::server;

#[tokio::main]
async fn main() {
    if let Err(error) = server::run().await {
        eprintln!("{}", StartupReport::failure(&error));
        std::process::exit(1);
    }
}
