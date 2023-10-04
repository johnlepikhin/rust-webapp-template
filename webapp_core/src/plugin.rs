use anyhow::Result;

pub trait Plugin: Send {
    fn webapp_initializer(&self, _service_config: &mut actix_web::web::ServiceConfig) {}
}

pub trait PluginMetadata {
    fn plugin_name(&self) -> &'static str;

    fn config_dump(&self) -> Result<Option<String>>;

    fn config_documentation(&self) -> Option<String>;

    fn new(configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized;

    fn init_plugin(&self, webapp: &crate::WebappCore) -> Result<Box<dyn Plugin>>;

    fn is_core(&self) -> bool {
        false
    }
}
