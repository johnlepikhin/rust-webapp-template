use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use slog::{o, Drain};
use slog_scope::error;
use structdoc::StructDoc;

mod config;

const CONFIG_DEFAULT_PATH: &str = "/etc/{{project-name}}.yaml";

// Example of subcommands
#[derive(Subcommand)]
enum CommandLine {
    /// Dump parsed config file. Helps to find typos
    DumpConfig,
    /// Print config file documentation
    ConfigDocumentation,
}

/// Example of simple cli program
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Application {
    /// Path to configuration file
    #[clap(short, default_value = CONFIG_DEFAULT_PATH)]
    config_path: String,
    /// Subcommand
    #[clap(subcommand)]
    command: CommandLine,
}

impl Application {
    fn init_syslog_logger(log_level: slog::Level) -> Result<slog_scope::GlobalLoggerGuard> {
        let logger = slog_syslog::SyslogBuilder::new()
            .facility(slog_syslog::Facility::LOG_USER)
            .level(log_level)
            .unix("/dev/log")
            .start()?;

        let logger = slog::Logger::root(logger.fuse(), o!());
        Ok(slog_scope::set_global_logger(logger))
    }

    fn init_env_logger() -> Result<slog_scope::GlobalLoggerGuard> {
        Ok(slog_envlogger::init()?)
    }

    fn init_logger(&self, config: &config::Config) -> Result<slog_scope::GlobalLoggerGuard> {
        if std::env::var("RUST_LOG").is_ok() {
            Self::init_env_logger()
        } else {
            Self::init_syslog_logger(config.log_level.into())
        }
    }

    fn config_documentation() {
        println!(
            "Configuration file format. Default path is {}\n\n{}",
            CONFIG_DEFAULT_PATH,
            crate::config::Config::document()
        )
    }

    fn run_command(&self) -> Result<()> {
        match &self.command {
            CommandLine::DumpConfig => {
                let config = config::Config::read(&self.config_path).expect("Config");
                let _logger_guard = self.init_logger(&config).expect("Logger");

                let config =
                    serde_yaml::to_string(&config).with_context(|| "Failed to dump config")?;
                println!("{}", config);
                Ok(())
            }
            CommandLine::ConfigDocumentation => {
                Self::config_documentation();
                Ok(())
            }
        }
    }

    pub fn run(&self) {
        if let Err(err) = self.run_command() {
            error!("Failed with error: {:#}", err);
        }
    }
}

fn main() {
    Application::parse().run();
}
