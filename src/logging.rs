use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Severity level for log entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// A single audit log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

/// Logger configuration.
#[derive(Debug, Clone)]
pub struct Logger {
    pub log_dir: PathBuf,
    pub min_level: LogLevel,
    pub max_file_size: u64,
}

impl Logger {
    /// Create a new logger that writes to `log_dir`.
    pub fn new(log_dir: PathBuf) -> Self {
        Self {
            log_dir,
            min_level: LogLevel::Info,
            max_file_size: 10 * 1024 * 1024, // 10 MB
        }
    }

    /// Log an event at the given level.
    pub fn log(&self, level: LogLevel, source: &str, message: &str) -> std::io::Result<()> {
        if (level as u8) < (self.min_level as u8) {
            return Ok(());
        }

        fs::create_dir_all(&self.log_dir)?;

        let entry = format!(
            "[{:?}] {} | {} | {}\n",
            level, chrono_now(), source, message
        );

        let log_file = self.log_dir.join("audit.log");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        file.write_all(entry.as_bytes())?;

        Ok(())
    }

    /// Rotate the log file if it exceeds `max_file_size`.
    pub fn rotate_if_needed(&self) -> std::io::Result<()> {
        let log_file = self.log_dir.join("audit.log");
        if log_file.exists() {
            let metadata = fs::metadata(&log_file)?;
            if metadata.len() > self.max_file_size {
                let backup = self.log_dir.join("audit.log.1");
                fs::rename(&log_file, &backup)?;
            }
        }
        Ok(())
    }
}

/// Convenience function: log an info-level entry.
pub fn info(source: &str, message: &str) -> std::io::Result<()> {
    let logger = Logger::new(PathBuf::from("/tmp/secure-lib-logs"));
    logger.log(LogLevel::Info, source, message)
}

/// Convenience function: log an error-level entry.
pub fn error(source: &str, message: &str) -> std::io::Result<()> {
    let logger = Logger::new(PathBuf::from("/tmp/secure-lib-logs"));
    logger.log(LogLevel::Error, source, message)
}

/// Convenience function: log a warning-level entry.
pub fn warn(source: &str, message: &str) -> std::io::Result<()> {
    let logger = Logger::new(PathBuf::from("/tmp/secure-lib-logs"));
    logger.log(LogLevel::Warning, source, message)
}

/// Set the global minimum log level.
pub fn set_log_level(level: LogLevel) {
    // In production this would set a global atomic
    let _ = level;
}

fn chrono_now() -> String {
    // Simplified timestamp for demo
    "2026-07-18T12:00:00Z".to_string()
}
