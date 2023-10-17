mod api_main;

use anyhow::Result;
use async_trait::async_trait;
use paperclip_actix::web;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;
use webapp_core::plugin::{Plugin, PluginMetadata};

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct Config {
    /// Some field in plugin config
    pub test_field: String,
    pub secret: webapp_yaml_config::secret::SecUtf8,
}

pub struct Metadata {
    configs_path: std::path::PathBuf,
}

#[async_trait]
impl PluginMetadata for Metadata {
    fn plugin_name(&self) -> &'static str {
        "test"
    }

    fn config_dump(&self) -> Result<Option<String>> {
        let config: webapp_yaml_config::yaml::Config<Config> =
            webapp_yaml_config::yaml::Config::new(&self.configs_path, self.plugin_name())?;
        config.as_yaml().map(Some)
    }

    fn config_documentation(&self) -> Option<String> {
        Some(Config::document().to_string())
    }

    fn new(configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            configs_path: configs_path.to_path_buf(),
        })
    }

    async fn init_plugin(&self) -> Result<Box<dyn Plugin>>
    where
        Self: Sized,
    {
        let plugin = PluginImpl::new(self)?;
        Ok(Box::new(plugin))
    }
}

pub struct PluginImpl {
    pub config: webapp_yaml_config::yaml::Config<Config>,
    pub zz: usize,
}

impl PluginImpl {
    pub fn new(metadata: &Metadata) -> Result<Self>
    where
        Self: Sized,
    {
        let config =
            webapp_yaml_config::yaml::Config::new(&metadata.configs_path, metadata.plugin_name())?;

        Ok(Self { config, zz: 1 })
    }
}

impl Plugin for PluginImpl {
    fn webapp_initializer(&self, service_config: &mut paperclip_actix::web::ServiceConfig) {
        let _ = service_config
            .route("/index.html", web::get().to(crate::api_main::index))
            .app_data(actix_web::web::Data::new(self.config.clone()));
    }
}
