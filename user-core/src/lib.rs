pub mod api;
pub mod db;
pub mod user;

use anyhow::Result;
use async_trait::async_trait;
use utoipa::OpenApi;
use webapp_core::plugin::{Plugin, PluginMetadata};

pub struct Metadata {}

#[async_trait]
impl PluginMetadata for Metadata {
    fn plugin_name(&self) -> &'static str {
        "user_core"
    }

    fn config_dump(&self) -> Result<Option<String>> {
        Ok(None)
    }

    fn config_documentation(&self) -> Option<String> {
        None
    }

    fn new(_configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
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
        let _ = service_config
            .service(crate::api::logout)
            .service(crate::api::user_list);

        #[derive(OpenApi)]
        #[openapi(
            paths(crate::api::logout, crate::api::user_list),
            components(schemas(crate::api::UserListUser))
        )]
        struct ApiDoc;

        ApiDoc::openapi()
    }
}
