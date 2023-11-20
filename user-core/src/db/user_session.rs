use anyhow::{anyhow, Result};
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use test_db::schema::user_session;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user_session)]
pub struct UserSession {
    pub id: i64,
    pub user_id: Option<i64>,
    pub token: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: chrono::DateTime<chrono::Utc>,
    pub requests_count: i64,
    pub last_address: ipnet::IpNet,
}

#[derive(Insertable)]
#[diesel(table_name = user_session)]
pub struct UserSessionNew {
    pub user_id: Option<i64>,
    token: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: chrono::DateTime<chrono::Utc>,
    pub requests_count: i64,
    pub last_address: ipnet::IpNet,
}

impl UserSessionNew {
    pub fn new(user: &crate::db::user::User, last_address: ipnet::IpNet) -> Self {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        Self {
            user_id: Some(user.id),
            token,
            create_date: chrono::Utc::now(),
            last_seen_date: chrono::Utc::now(),
            requests_count: 0,
            last_address,
        }
    }

    pub fn with_user(self, user_id: i64) -> Self {
        Self {
            user_id: Some(user_id),
            ..self
        }
    }
}

impl UserSession {
    pub fn new(
        db: &mut diesel::PgConnection,
        user: &crate::db::user::User,
        last_address: ipnet::IpNet,
    ) -> Result<UserSession> {
        let r = diesel::insert_into(user_session::dsl::user_session)
            .values(UserSessionNew::new(user, last_address))
            .get_result(db)
            .map_err(|err| anyhow!("Failed to add user session: {err}"))?;
        Ok(r)
    }
}
