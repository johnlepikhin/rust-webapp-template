use anyhow::{anyhow, Result};
use diesel::prelude::*;
use test_db::schema::user;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i64,
    pub object_id: i64,
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
            .get_result(db)
            .map_err(|err| anyhow!("Failed to add user: {err}"))?;
        Ok(r)
    }

    pub fn of_username(db: &mut diesel::PgConnection, username: &str) -> Result<User> {
        let r = user::dsl::user
            .filter(user::dsl::username.eq(username))
            .get_result(db)
            .map_err(|err| anyhow!("Failed to get user: {err}"))?;
        Ok(r)
    }

    pub fn logged_in(&self, db: &mut diesel::PgConnection) -> Result<()> {
        diesel::update(user::dsl::user.find(self.id))
            .set((
                user::dsl::last_seen_date.eq(Some(chrono::Utc::now())),
                user::dsl::login_count.eq(user::dsl::login_count + 1),
            ))
            .execute(db)
            .map_err(|err| anyhow!("Failed to update user: {err}"))?;
        Ok(())
    }
}
