use anyhow::Result;
use async_trait::async_trait;
use utoipa::OpenApi;

pub trait Plugin: Send {
    fn webapp_initializer(
        &self,
        _service_config: &mut actix_web::web::ServiceConfig,
    ) -> utoipa::openapi::OpenApi {
        #[derive(OpenApi)]
        #[openapi()]
        struct ApiDoc;

        ApiDoc::openapi()
    }
}

#[async_trait]
pub trait PluginMetadata {
    fn plugin_name(&self) -> &'static str;

    fn config_dump(&self) -> Result<Option<String>>;

    fn config_documentation(&self) -> Option<String>;

    fn new(configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized;

    async fn init_plugin(&self) -> Result<Box<dyn Plugin>>;

    fn is_core(&self) -> bool {
        false
    }
}
