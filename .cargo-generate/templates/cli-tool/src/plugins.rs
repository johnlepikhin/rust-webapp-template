use std::collections::HashSet;

use anyhow::{bail, Result};
use webapp_core::plugin::PluginMetadata;

fn list(configs_path: &std::path::Path) -> Result<Vec<Box<dyn PluginMetadata>>> {
    // TODO simplify casting
    let r = vec![
        // Box::new({{db_plugin}}::Metadata::new(configs_path)?) as Box<dyn PluginMetadata>,
    ];
    Ok(r)
}

pub fn register(configs_path: &std::path::Path) -> Result<Vec<Box<dyn PluginMetadata>>> {
    let list = list(configs_path)?;
    let mut names = HashSet::new();
    for metadata in &list {
        let name = metadata.plugin_name();
        if names.contains(name) {
            bail!("Plugin with name {name:?} already registered")
        }
        let _ = names.insert(name);
    }
    Ok(list)
}
