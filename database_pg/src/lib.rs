pub mod secstr;
pub mod sync;

use anyhow::{anyhow, Result};
use deadpool_diesel::postgres::Manager;
use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc)]
pub struct Config {
    /// Postgres DB URL, see https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
    pub database_url: webapp_yaml_config::secret::Secret,
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
            config.config.database_url.unsecure()?,
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
        let span = tracing::Span::current();
        self.with_connection(|conn| {
            let _span_guard = span.entered();
            conn.build_transaction().repeatable_read().run(f)
        })
        .await
    }
}

#[macro_export]
macro_rules! make_id {
    ($id_name:ident) => {
        #[derive(Debug, Eq, Hash, PartialEq, Clone, diesel::expression::AsExpression)]
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        pub struct $id_name(i64);

        impl Into<i64> for $id_name {
            fn into(self) -> i64 {
                self.0
            }
        }

        impl<DB> Queryable<diesel::sql_types::BigInt, DB> for $id_name
        where
            DB: diesel::backend::Backend,
            i64: diesel::deserialize::FromSql<diesel::sql_types::BigInt, DB>,
        {
            type Row = i64;

            fn build(v: i64) -> diesel::deserialize::Result<Self> {
                Ok($id_name(v))
            }
        }

        impl<DB> diesel::serialize::ToSql<diesel::sql_types::BigInt, DB> for $id_name
        where
            DB: diesel::backend::Backend,
            i64: diesel::serialize::ToSql<diesel::sql_types::BigInt, DB>,
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, DB>,
            ) -> diesel::serialize::Result {
                <i64>::to_sql(&self.0, out)
            }
        }
    };
}
