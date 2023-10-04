pub mod config;
pub mod plugin;

use actix_web::{App, HttpServer};
use anyhow::{bail, Result};
use slog::{o, Drain};
use std::sync::{Arc, Mutex};

pub struct WebappCore {
    pub config: webapp_yaml_config::yaml::Config<crate::config::Config>,
    _logger_guard: slog_scope::GlobalLoggerGuard,
}

impl WebappCore {
    fn plugin_name() -> &'static str {
        "core"
    }

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

    fn init_logger(config: &crate::config::Config) -> Result<slog_scope::GlobalLoggerGuard> {
        if std::env::var("RUST_LOG").is_ok() {
            Self::init_env_logger()
        } else {
            Self::init_syslog_logger(config.log_level.into())
        }
    }

    pub fn new(configs_path: &std::path::Path) -> Result<Self> {
        let config = webapp_yaml_config::yaml::Config::new(configs_path, Self::plugin_name())?;
        let logger_guard = config.with_config(|config| Self::init_logger(&config))?;

        Ok(Self {
            config,
            _logger_guard: logger_guard,
        })
    }

    pub async fn run(self, plugins: Vec<Arc<Mutex<Box<dyn crate::plugin::Plugin>>>>) -> Result<()> {
        let (bind_address, bind_port) = {
            self.config
                .with_config(|c| Ok((c.bind_address.clone(), c.bind_port)))?
        };

        HttpServer::new(move || {
            let mut app = App::new();
            for plugin in &plugins {
                let guarded_plugin = plugin.lock().unwrap();
                app = app
                    .configure(|service_config| guarded_plugin.webapp_initializer(service_config))
            }
            app
        })
        .bind((bind_address, bind_port))?
        .run()
        .await?;
        Ok(())
    }
}

pub struct WebappCoreMetadata {
    configs_path: std::path::PathBuf,
}

impl crate::plugin::PluginMetadata for WebappCoreMetadata {
    fn plugin_name(&self) -> &'static str {
        WebappCore::plugin_name()
    }

    fn config_dump(&self) -> Result<Option<String>> {
        let config: webapp_yaml_config::yaml::Config<crate::config::Config> =
            webapp_yaml_config::yaml::Config::new(&self.configs_path, self.plugin_name())?;
        config.as_yaml().map(Some)
    }

    fn config_documentation(&self) -> Option<String> {
        use structdoc::StructDoc;
        Some(crate::config::Config::document().to_string())
    }

    fn new(configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            configs_path: configs_path.to_path_buf(),
        })
    }

    fn init_plugin(&self, _webapp: &crate::WebappCore) -> Result<Box<dyn plugin::Plugin>> {
        bail!("Core cannot be ever initialized as usual plugin")
    }

    fn is_core(&self) -> bool {
        true
    }
}
