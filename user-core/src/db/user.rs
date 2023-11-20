use anyhow::Result;
use diesel::prelude::*;
use test_db::schema::user;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i64,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: Option<chrono::DateTime<chrono::Utc>>,
    pub login_count: i64,
    pub username: String,
    pub person: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct UserNew {
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub last_seen_date: Option<chrono::DateTime<chrono::Utc>>,
    pub login_count: i64,
    pub username: String,
    pub person: String,
}

impl User {
    pub fn new(db: &mut diesel::PgConnection, username: String, person: String) -> Result<User> {
        let r = diesel::insert_into(user::dsl::user)
            .values(UserNew {
                create_date: chrono::Utc::now(),
                last_seen_date: None,
                login_count: 0,
                username,
                person,
            })
            .get_result(db)?;
        Ok(r)
    }
}
