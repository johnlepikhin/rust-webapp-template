use std::sync::Arc;

use anyhow::Result;
use structdoc::StructDoc;
use webapp_core::plugin::{Plugin, PluginMetadata};

pub struct MainDBMetadata {
    configs_path: std::path::PathBuf,
}

impl PluginMetadata for MainDBMetadata {
    fn plugin_name(&self) -> &'static str {
        "main_db"
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

    fn init_plugin(&self) -> Result<Box<dyn Plugin>>
    where
        Self: Sized,
    {
        let plugin = MainDB::new(self)?;
        Ok(Box::new(plugin))
    }
}

pub struct MainDB {
    pub pool: Arc<database_pg::Pool>,
}

impl MainDB {
    pub fn new(metadata: &MainDBMetadata) -> Result<Self>
    where
        Self: Sized,
    {
        let pool = database_pg::Pool::new(metadata.plugin_name(), &metadata.configs_path)?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

impl Plugin for MainDB {
    fn webapp_initializer(&self, service_config: &mut actix_web::web::ServiceConfig) {
        let _ = service_config.app_data(self.pool.clone());
    }
}
