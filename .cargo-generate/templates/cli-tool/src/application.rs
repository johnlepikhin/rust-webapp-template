use anyhow::Result;
use clap::Args;
use webapp_core::plugin::PluginMetadata;

#[derive(Args)]
pub struct Run {}

impl Run {
    pub async fn run(
        &self,
        configs_path: &std::path::Path,
        _plugins_meta: &[Box<dyn webapp_core::plugin::PluginMetadata>],
    ) -> Result<()> {
        Ok(())
    }
}
