use std::path::Path;
use std::sync::{Mutex, TryLockError};

use tracing_appender::rolling::{InitError, RollingFileAppender, Rotation};

pub const LOG_FILES_KEPT: usize = 14;

const LOG_FILE_PREFIX: &str = "miner.log";

pub fn daily_files(dir: &Path) -> std::result::Result<RollingFileAppender, InitError> {
    let files = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(LOG_FILE_PREFIX)
        .max_log_files(LOG_FILES_KEPT)
        .build(dir)?;

    keep_private(dir);

    Ok(files)
}

// ! The appender creates each new day's file with the default mode: the 0700 folder hides it from other users.
fn keep_private(dir: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with(LOG_FILE_PREFIX)
            {
                let _ =
                    std::fs::set_permissions(entry.path(), std::fs::Permissions::from_mode(0o600));
            }
        }
    }

    #[cfg(not(unix))]
    let _ = dir;
}

static FILE_WRITER: Mutex<Option<Box<dyn Send>>> = Mutex::new(None);

#[must_use = "the log file is written to the end when this is dropped"]
pub struct FileWriter(());

impl Drop for FileWriter {
    fn drop(&mut self) {
        close_file_writer();
    }
}

pub fn close_file_writer() {
    drop(take_file_writer());
}

pub fn keep_file_writer(guard: impl Send + 'static) -> FileWriter {
    *crate::util::locked(&FILE_WRITER) = Some(Box::new(guard));

    FileWriter(())
}

fn take_file_writer() -> Option<Box<dyn Send>> {
    match FILE_WRITER.try_lock() {
        Ok(mut writer) => writer.take(),
        Err(TryLockError::Poisoned(writer)) => writer.into_inner().take(),
        Err(TryLockError::WouldBlock) => None,
    }
}

pub fn log_panics() {
    static HOOK: std::sync::Once = std::sync::Once::new();

    HOOK.call_once(|| {
        let previous = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |info| {
            tracing::error!("panic: {info}");

            // ! With `panic = "abort"` the process ends after this hook: the file writer must flush first.
            if cfg!(panic = "abort") {
                drop(take_file_writer());
            }

            previous(info);
        }));
    });
}
