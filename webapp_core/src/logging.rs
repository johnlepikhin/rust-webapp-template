use anyhow::Result;
use serde::{Deserialize, Serialize};
use slog::{o, Drain};
use sloggers::Config;
use structdoc::Documentation;
use structdoc::StructDoc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig(sloggers::LoggerConfig);

impl structdoc::StructDoc for LoggerConfig {
    fn document() -> Documentation {
        Documentation::leaf("Logger configuration, see. https://docs.rs/sloggers/latest/sloggers/enum.LoggerConfig.html")
    }
}

impl From<sloggers::LoggerConfig> for LoggerConfig {
    fn from(v: sloggers::LoggerConfig) -> Self {
        Self(v)
    }
}

impl std::ops::Deref for LoggerConfig {
    type Target = sloggers::LoggerConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, StructDoc)]
pub struct Loggers(Vec<LoggerConfig>);

impl Loggers {
    fn build_logger(&self) -> slog::Logger {
        let mut combined_logger = None;
        for logger_config in self.0.iter() {
            let logger = logger_config.build_logger().unwrap();
            if let Some(pred) = combined_logger.take() {
                combined_logger = Some(slog::Logger::root(
                    slog::Duplicate::new(logger, pred).fuse(),
                    o!(),
                ));
            } else {
                combined_logger = Some(logger);
            }
        }
        let logger = combined_logger.take().unwrap();
        let logger = slog_async::Async::default(logger);
        slog::Logger::root(logger.fuse(), o!())
    }

    pub fn run(&self) -> Result<slog_scope::GlobalLoggerGuard> {
        let logger = self.build_logger();
        let guard = sloggers::set_stdlog_logger(logger)?;
        Ok(guard)
    }
}

impl Default for Loggers {
    fn default() -> Self {
        Self(vec![sloggers::LoggerConfig::Terminal(
            sloggers::terminal::TerminalLoggerConfig::new(),
        )
        .into()])
    }
}
