mod apidoc;
pub mod config;
pub mod logging;
pub mod plugin;
pub mod secstr;

use actix_web::{dev::Service, App, HttpServer};
use anyhow::{bail, Result};
use async_trait::async_trait;
use slog::{o, FnValue};
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use utoipa_swagger_ui::SwaggerUi;

pub const SESSION_COOKIE_NAME: &str = "session";

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
        let config: webapp_yaml_config::yaml::Config<crate::config::Config> =
            webapp_yaml_config::yaml::Config::new(configs_path, Self::plugin_name())?;
        let logger_guard = Self::init_logger(&config.config)?;

        Ok(Self {
            config,
            _logger_guard: logger_guard,
        })
    }

    fn get_cors(config: &crate::config::Config) -> actix_cors::Cors {
        if let Some(cors_config) = config.cors.clone() {
            let mut cors = actix_cors::Cors::default();
            for origin in &cors_config.origins {
                cors = cors.allowed_origin(origin.as_str());
            }
            cors
        } else {
            actix_cors::Cors::permissive()
        }
    }

    pub async fn run(self, plugins: Vec<Arc<Mutex<Box<dyn crate::plugin::Plugin>>>>) -> Result<()> {
        let config = Arc::new(self.config);
        let app_config = config.clone();
        HttpServer::new(move || {
            let logger = slog_scope::logger()
                .new(o!("request_id" => FnValue(|_| REQUESTS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))));
            let cors = Self::get_cors(&app_config.config);
            let mut apidoc = crate::apidoc::new();
            let mut app = App::new()
                .wrap(actix_web::middleware::Logger::default())
                .wrap_fn(move |req, srv| {
                    slog_scope_futures::SlogScope::new(logger.clone(), srv.call(req))
                })
                .wrap(cors);
            for plugin in &plugins {
                let guarded_plugin = plugin.lock().unwrap();
                app = app
                    .configure(|service_config| {
                        let plugin_apidoc = guarded_plugin.webapp_initializer(service_config);
                        apidoc.merge(plugin_apidoc)
                    })
            }
            if let Some(openapi) = &app_config.config.openapi {
                app = app
                    .service(
                        SwaggerUi::new(format!("{}/{{_:.*}}", openapi.swagger_uri))
                            .url(openapi.spec_uri.clone(), apidoc),
                    );
            }
            app
            })
            .keep_alive(config.config.keep_alive)
            .shutdown_timeout(config.config.shutdown_timeout.as_secs())
            .bind((config.config.bind_address.clone(), config.config.bind_port))?
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

        let sample_config = serde_yaml::to_string(&crate::config::Config::default()).unwrap();

        let r = format!(
            "{}\n\nExample config:\n\n{}",
            crate::config::Config::document(),
            sample_config
        );
        Some(r)
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
