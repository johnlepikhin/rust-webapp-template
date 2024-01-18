use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub struct Pool {
    pub config: webapp_yaml_config::yaml::Config<crate::Config>,
    pool: diesel::r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Pool {
    pub fn new(plugin_name: &'static str, configs_path: &std::path::Path) -> Result<Self> {
        let config: webapp_yaml_config::yaml::Config<crate::Config> =
            webapp_yaml_config::yaml::Config::new(configs_path, plugin_name)?;

        let manager =
            ConnectionManager::<PgConnection>::new(config.config.database_url.unsecure()?);
        let pool = diesel::r2d2::Pool::builder().build(manager)?;

        Ok(Self { config, pool })
    }

    pub fn with_connection<RESULT, F>(&self, f: F) -> Result<RESULT>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<RESULT> + Send + 'static,
        RESULT: Send + 'static,
    {
        let mut conn = self.pool.get()?;
        f(&mut conn).map_err(|err| anyhow!("{}", err))
    }

    pub fn with_transaction<RESULT, F>(&self, f: F) -> Result<RESULT>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<RESULT> + Send + 'static,
        RESULT: Send + 'static,
    {
        self.with_connection(|conn| conn.build_transaction().read_committed().run(f))
    }
}
