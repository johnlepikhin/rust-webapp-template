pub mod config;
pub mod logging;
pub mod plugin;

use actix_web::{dev::Service, App, HttpServer};
use anyhow::{bail, Result};
use async_trait::async_trait;
use paperclip::actix::OpenApiExt;
use slog::{o, FnValue};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};

static REQUESTS_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct WebappCore {
    pub config: webapp_yaml_config::yaml::Config<crate::config::Config>,
    _logger_guard: slog_scope::GlobalLoggerGuard,
}

impl WebappCore {
    fn plugin_name() -> &'static str {
        "core"
    }

    fn init_logger(config: &crate::config::Config) -> Result<slog_scope::GlobalLoggerGuard> {
        if std::env::var("RUST_LOG").is_ok() {
            Ok(slog_envlogger::init()?)
        } else {
            config.loggers.run()
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
            let logger = slog_scope::logger()
                .new(o!("request_id" => FnValue(|_| REQUESTS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))));
            let mut app = App::new()
                .wrap(actix_web::middleware::Logger::default())
                .wrap_fn(move |req, srv| {
                    slog_scope_futures::SlogScope::new(logger.clone(), srv.call(req))
                })
                .wrap_api()
                ;
            for plugin in &plugins {
                let guarded_plugin = plugin.lock().unwrap();
                app = app
                    .configure(|service_config| guarded_plugin.webapp_initializer(service_config))
            }

            if let Some(openapi) = self.config.with_config(|config| Ok(config.openapi.clone())).unwrap() {
                let app = app
                    .with_json_spec_at(&openapi.spec_uri);
                let app = match openapi.swagger_uri {
                    Some(v) => app.with_swagger_ui_at(&v),
                    None => app,
                };
                app.build()
            } else {
                app.build()
            }
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

#[async_trait]
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

    async fn init_plugin(&self) -> Result<Box<dyn plugin::Plugin>> {
        bail!("Core cannot be ever initialized as usual plugin")
    }

    fn is_core(&self) -> bool {
        true
    }
}
