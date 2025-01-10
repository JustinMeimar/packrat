use std::sync::{OnceLock, Mutex};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

static LOGGER: OnceLock<Mutex<File>> = OnceLock::new();

fn init_logger() -> &'static Mutex<File> {
    LOGGER.get_or_init(|| {
        Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("debug.log")
                .expect("Unable to open log file"),
        )
    })
}

pub fn debug_log(message: &str) {
    let logger = init_logger();
    let mut logger = logger.lock().unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    writeln!(logger, "[{}] {}", timestamp, message)
        .expect("Unable to write to log file");
}

