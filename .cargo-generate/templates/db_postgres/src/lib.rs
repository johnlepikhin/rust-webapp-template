pub mod db;
pub mod schema;

use anyhow::Result;
use async_trait::async_trait;
use structdoc::StructDoc;

pub struct Metadata {
    configs_path: std::path::PathBuf,
}

#[async_trait]
impl webapp_core::plugin::PluginMetadata for Metadata {
    fn plugin_name(&self) -> &'static str {
        "{{project-name}}"
    }

    fn config_dump(&self) -> Result<Option<String>> {
        let config: webapp_yaml_config::yaml::Config<database_pg::Config> =
            webapp_yaml_config::yaml::Config::new(&self.configs_path, self.plugin_name())?;
        config.as_yaml().map(Some)
    }

    fn config_documentation(&self) -> Option<String> {
        Some(database_pg::Config::document().to_string())
    }

    fn new(configs_path: &std::path::Path) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            configs_path: configs_path.to_path_buf(),
        })
    }

    async fn init_plugin(&self) -> Result<Box<dyn webapp_core::plugin::Plugin>>
    where
        Self: Sized,
    {
        let plugin = crate::db::DB::new(self).await?;
        Ok(Box::new(plugin))
    }
}
