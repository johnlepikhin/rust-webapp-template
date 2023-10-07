use anyhow::Result;
use clap::Args;
use std::sync::{Arc, Mutex};

#[derive(Args)]
pub struct Run {
    #[clap(short, long, default_value_t = false)]
    foreground: bool,
}

impl Run {
    pub async fn run(
        &self,
        configs_path: &std::path::Path,
        plugins_meta: &[Box<dyn webapp_core::plugin::PluginMetadata>],
    ) -> Result<()> {
        let webapp = webapp_core::WebappCore::new(configs_path)?;

        let mut plugins = Vec::new();
        for plugin_meta in plugins_meta {
            if plugin_meta.is_core() {
                continue;
            }

            let plugin = plugin_meta.init_plugin().await?;
            plugins.push(Arc::new(Mutex::new(plugin)))
        }

        webapp.run(plugins).await
    }
}
