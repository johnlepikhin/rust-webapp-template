pub mod api;
pub mod config;
mod db;

use anyhow::Result;
use async_trait::async_trait;
use structdoc::StructDoc;
use utoipa::OpenApi;
use webapp_core::plugin::{Plugin, PluginMetadata};

pub struct Metadata {
    configs_path: std::path::PathBuf,
}

#[async_trait]
impl PluginMetadata for Metadata {
    fn plugin_name(&self) -> &'static str {
        "{{project_name}}"
    }

    fn config_dump(&self) -> Result<Option<String>> {
        let config: webapp_yaml_config::yaml::Config<crate::config::Config> =
            webapp_yaml_config::yaml::Config::new(&self.configs_path, self.plugin_name())?;
        config.as_yaml().map(Some)
    }

    fn config_documentation(&self) -> Option<String> {
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

    async fn init_plugin(&self) -> Result<Box<dyn Plugin>>
    where
        Self: Sized,
    {
        let plugin = PluginImpl::new(self)?;
        Ok(Box::new(plugin))
    }
}

pub struct PluginImpl {}

impl PluginImpl {
    pub fn new(_metadata: &Metadata) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

impl Plugin for PluginImpl {
    fn webapp_initializer(
        &self,
        service_config: &mut actix_web::web::ServiceConfig,
    ) -> utoipa::openapi::OpenApi {
        let _ = service_config.service(crate::api::login);

        #[derive(OpenApi)]
        #[openapi(
            paths(crate::api::login),
            components(schemas(crate::api::LoginRequest, crate::api::LoginResponse))
        )]
        struct ApiDoc;

        ApiDoc::openapi()
    }
}
