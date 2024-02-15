mod apidoc;
pub mod config;
pub mod logging;
pub mod plugin;
pub mod secstr;

use actix_web::{App, HttpServer};
use anyhow::{bail, Result};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tracing_actix_web::TracingLogger;
use utoipa_swagger_ui::SwaggerUi;

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct WebappCore {
    pub config: webapp_yaml_config::yaml::Config<crate::config::Config>,
}

impl WebappCore {
    fn plugin_name() -> &'static str {
        "core"
    }

    pub fn new(configs_path: &std::path::Path) -> Result<Self> {
        let config: webapp_yaml_config::yaml::Config<crate::config::Config> =
            webapp_yaml_config::yaml::Config::new(configs_path, Self::plugin_name())?;
        config.logger.init()?;

        Ok(Self { config })
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
            let cors = Self::get_cors(&app_config.config);
            let mut apidoc = crate::apidoc::new();
            let mut app = App::new()
                .wrap_fn(|req, srv| {
                    use actix_web::{
                        dev::Service,
                        http::header::{HeaderName, HeaderValue},
                        HttpMessage,
                    };
                    use tracing_actix_web::RequestId;
                    let request_id = req
                        .extensions()
                        .get::<RequestId>()
                        .copied()
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    let res = srv.call(req);
                    async move {
                        tracing::debug!("New request");
                        let mut res = res.await?;
                        res.headers_mut().insert(
                            HeaderName::from_static("x-request-id"),
                            // this unwrap never fails, since UUIDs are valid ASCII strings
                            HeaderValue::from_str(&request_id.to_string()).unwrap(),
                        );
                        Ok(res)
                    }
                })
                .wrap(TracingLogger::default())
                .wrap(actix_web_opentelemetry::RequestTracing::new())
                .wrap(cors);
            for plugin in &plugins {
                let guarded_plugin = plugin.lock().unwrap();
                app = app.configure(|service_config| {
                    let plugin_apidoc = guarded_plugin.webapp_initializer(service_config);
                    apidoc.merge(plugin_apidoc)
                })
            }
            if let Some(openapi) = &app_config.config.openapi {
                app = app.service(
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
