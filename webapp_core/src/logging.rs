use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Debug, Clone, Serialize, Deserialize, StructDoc, Default)]
pub enum Logger {
    /// Use journald for logging
    #[serde(rename = "journald")]
    #[default]
    Journald,
}

impl Logger {
    pub fn init(&self) -> Result<()> {
        if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::fmt::init();
            return Ok(());
        }

        use tracing_subscriber::prelude::*;
        match self {
            Logger::Journald => {
                let layer = tracing_journald::layer()
                    .map_err(|err| anyhow!("Cannot connect to journald: {err}"))?;
                tracing_subscriber::registry().with(layer).init();
                Ok(())
            }
        }
    }
}
