use anyhow::Result;
use std::sync::Arc;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!();

pub struct DB {
    pub pool: Arc<database_pg::Pool>,
}

impl DB {
    pub async fn new(metadata: &crate::Metadata) -> Result<Self>
    where
        Self: Sized,
    {
        use webapp_core::plugin::PluginMetadata;
        let pool = database_pg::Pool::new(metadata.plugin_name(), &metadata.configs_path)?;

        let r = Self {
            pool: Arc::new(pool),
        };

        r.run_pending_migrations().await?;

        Ok(r)
    }

    async fn run_pending_migrations(&self) -> Result<()> {
        self.pool
            .with_transaction(|conn| {
                use diesel_migrations::MigrationHarness;
                let mut harness = diesel_migrations::HarnessWithOutput::write_to_stdout(conn);
                harness
                    .run_pending_migrations(MIGRATIONS)
                    .expect("Failed to run pending migrations");
                Ok(())
            })
            .await
    }
}

impl webapp_core::plugin::Plugin for DB {
    fn webapp_initializer(&self, service_config: &mut paperclip_actix::web::ServiceConfig) {
        let _ = service_config.app_data(self.pool.clone());
    }
}
