use anyhow::{anyhow, Result};
// TODO использовать мьютексы tokio
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Config<CONFIG> {
    pub name: &'static str,
    pub path: std::path::PathBuf,
    pub config: Arc<Mutex<CONFIG>>,
}

impl<CONFIG> Config<CONFIG>
where
    CONFIG: serde::de::DeserializeOwned + serde::Serialize,
{
    fn read(path: &std::path::Path) -> Result<CONFIG> {
        let config = std::fs::read_to_string(path)
            .map_err(|err| anyhow!("Failed to load config file {path:?}: {err}"))?;
        let config: CONFIG = serde_yaml::from_str(&config)
            .map_err(|err| anyhow!("Failed to parse config file {path:?}: {err}"))?;

        Ok(config)
    }

    pub fn new(configs_path: &std::path::Path, name: &'static str) -> Result<Self> {
        let path = configs_path.join(format!("{}.yaml", name));
        let config = Self::read(&path)?;
        Ok(Self {
            name,
            path,
            config: Arc::new(Mutex::new(config)),
        })
    }

    pub fn with_config<R, CB>(&self, cb: CB) -> Result<R>
    where
        CB: FnOnce(MutexGuard<CONFIG>) -> Result<R>,
    {
        let v = self
            .config
            .lock()
            .map_err(|_err| anyhow!("Cannot lock mutex for config {:?}", self.name))?;
        cb(v)
    }

    pub fn as_yaml(&self) -> Result<String> {
        self.with_config(|config| Ok(serde_yaml::to_string(&*config)?))
    }
}
