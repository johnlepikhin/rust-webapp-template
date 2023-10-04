use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Clone, Copy, Serialize, Deserialize, StructDoc)]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for slog::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Critical => slog::Level::Critical,
            LogLevel::Error => slog::Level::Error,
            LogLevel::Warning => slog::Level::Warning,
            LogLevel::Info => slog::Level::Info,
            LogLevel::Debug => slog::Level::Debug,
            LogLevel::Trace => slog::Level::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, StructDoc)]
pub struct Config {
    /// Bind web application to specified address. For example, "127.0.0.1"
    pub bind_address: String,
    /// Bind web application to specified port. For example, 8080
    pub bind_port: u16,
    /// Max log level for syslog mode
    pub log_level: LogLevel,
}
