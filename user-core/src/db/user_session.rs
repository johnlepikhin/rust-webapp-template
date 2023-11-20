use anyhow::Result;
use diesel::prelude::*;
use test_db::schema::user_session;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user_session)]
pub struct UserSession {
    pub id: i64,
    pub user_id: Option<i64>,
    token: String,
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
    pub fn new(token: String, last_address: ipnet::IpNet) -> Self {
        Self {
            user_id: None,
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
    pub fn new(db: &mut diesel::PgConnection, session: UserSessionNew) -> Result<UserSession> {
        let r = diesel::insert_into(user_session::dsl::user_session)
            .values(session)
            .get_result(db)?;
        Ok(r)
    }
}
