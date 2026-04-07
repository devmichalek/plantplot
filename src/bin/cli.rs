
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;

fn main() {
    let file_appender = rolling::daily("/tmp/plantplot", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = fmt()
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG
        .with_writer(non_blocking) // write to file
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Example logs
    info!("Server started");
    error!("Something went wrong");

    // simulate work
    std::thread::sleep(std::time::Duration::from_secs(1));
}