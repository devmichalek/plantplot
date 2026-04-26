
use tracing::{info, error};

fn main() {
    let tracing_worker_guard = plantplot::core::tracing::initialize_tracing("cli");
    info!("Successfully initialized tracing module!");
}