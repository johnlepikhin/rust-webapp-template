use anyhow::{anyhow, Result};
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use {{db_plugin}}::schema::user_session;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user_session)]
pub struct UserSession {
    pub id: i64,
    pub user_id: Option<i64>,
    pub token: database_pg::secstr::SecUtf8,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: chrono::DateTime<chrono::Utc>,
    pub requests_count: i64,
    pub last_address: ipnet::IpNet,
}

#[derive(Insertable)]
#[diesel(table_name = user_session)]
pub struct UserSessionNew {
    pub user_id: Option<i64>,
    token: database_pg::secstr::SecUtf8,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: chrono::DateTime<chrono::Utc>,
    pub requests_count: i64,
    pub last_address: ipnet::IpNet,
}

impl UserSession {
    pub fn new(
        db: &mut diesel::PgConnection,
        user: &crate::db::user::User,
        last_address: ipnet::IpNet,
    ) -> Result<UserSession> {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let token = database_pg::secstr::SecUtf8::from(token);
        let r = diesel::insert_into(user_session::dsl::user_session)
            .values(UserSessionNew {
                user_id: Some(user.id),
                token,
                create_date: chrono::Utc::now(),
                last_seen_date: chrono::Utc::now(),
                requests_count: 0,
                last_address,
            })
            .get_result(db)
            .map_err(|err| anyhow!("Failed to add user session: {err}"))?;
        Ok(r)
    }
}
