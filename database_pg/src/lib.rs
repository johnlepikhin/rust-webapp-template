pub mod secstr;

use anyhow::{anyhow, Result};
use deadpool_diesel::postgres::Manager;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc)]
pub struct Config {
    /// Postgres DB URL, see https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
    pub database_url: webapp_yaml_config::secret::SecUtf8,
    /// Maximum number of connections to keep opened
    pub max_connections: usize,
}

pub struct Pool {
    pub config: webapp_yaml_config::yaml::Config<Config>,
    pool: deadpool_diesel::postgres::Pool,
}

impl Pool {
    pub fn new(plugin_name: &'static str, configs_path: &std::path::Path) -> Result<Self> {
        let config: webapp_yaml_config::yaml::Config<Config> =
            webapp_yaml_config::yaml::Config::new(configs_path, plugin_name)?;

        let manager = Manager::new(
            config.config.database_url.unsecure(),
            deadpool_diesel::Runtime::Tokio1,
        );
        use deadpool_diesel::postgres::Pool;
        let pool = Pool::builder(manager)
            .max_size(config.config.max_connections)
            .build()?;

        Ok(Self { config, pool })
    }

    pub async fn with_connection<RESULT, F>(&self, f: F) -> Result<RESULT>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<RESULT> + Send + 'static,
        RESULT: Send + 'static,
    {
        let conn = self.pool.get().await?;
        conn.interact(f).await.map_err(|err| anyhow!("{}", err))?
    }

    pub async fn with_transaction<RESULT, F>(&self, f: F) -> Result<RESULT>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<RESULT> + Send + 'static,
        RESULT: Send + 'static,
    {
        self.with_connection(|conn| conn.build_transaction().read_committed().run(f))
            .await
    }
}
