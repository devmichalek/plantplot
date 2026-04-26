
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::{rolling, non_blocking};

pub fn initialize_tracing(filename_prefix: &str) -> non_blocking::WorkerGuard {
    let filename = filename_prefix.to_string() + ".log";
    let file_appender = rolling::daily("/tmp/plantplot", filename);
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = fmt()
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    return worker_guard;
}
