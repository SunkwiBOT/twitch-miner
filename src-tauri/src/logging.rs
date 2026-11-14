use super::*;

pub(super) fn init_logging(
    dir: &Path,
    log_buffer: miner_core::logs::LogBuffer,
) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let logs = dir.join("logs");
    let _ = miner_core::util::private_dir(&logs);

    // ! An unwritable data folder must reach the startup problem screen, not end the app.
    let (file, guard) = match miner_core::logs::daily_files(&logs) {
        Ok(file) => {
            let (file, guard) = tracing_appender::non_blocking(file);

            (Some(file), Some(guard))
        }
        Err(_) => (None, None),
    };
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,hyper=warn,reqwest=warn"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_writer(miner_core::logs::Redacted(std::io::stdout)),
        )
        .with(file.map(|file| {
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(miner_core::logs::Redacted(file))
        }))
        .with(miner_core::logs::LogLayer { buffer: log_buffer })
        .try_init();

    guard
}
