use anyhow::{anyhow, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use rand_core::OsRng;
use secstr::SecStr;
use test_db::schema::user_password;

fn hash_password(password: &secstr::SecStr) -> Result<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.unsecure(), &salt)
        .map_err(|err| anyhow!("Failed to hash password: {err}"))?;
    Ok(hash.to_string())
}

fn validate_password(hash: &str, password: &secstr::SecStr) -> Result<()> {
    let argon2 = Argon2::default();
    let hash =
        PasswordHash::new(hash).map_err(|err| anyhow!("Failed to parse password hash: {err}"))?;
    argon2
        .verify_password(password.unsecure(), &hash)
        .map_err(|err| anyhow!("Failed to verify password: {err}"))
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user_password)]
pub struct UserPassword {
    pub id: i64,
    pub user_id: i64,
    pub last_updated_date: chrono::DateTime<chrono::Utc>,
    password_hash: String,
}

impl UserPassword {
    pub fn update_password(&self, db: &mut diesel::PgConnection, password: &SecStr) -> Result<()> {
        let password_hash = hash_password(password)?;
        diesel::update(user_password::dsl::user_password.find(self.id))
            .set((
                user_password::dsl::last_updated_date.eq(chrono::Utc::now()),
                user_password::dsl::password_hash.eq(password_hash),
            ))
            .execute(db)
            .map_err(|err| anyhow!("Failed to update user password: {err}"))?;
        Ok(())
    }

    pub fn validate_password(&self, password: &SecStr) -> Result<()> {
        validate_password(&self.password_hash, password)
    }

    pub fn of_user(
        db: &mut diesel::PgConnection,
        user: &user_core::db::user::User,
    ) -> Result<Self> {
        let r = user_password::dsl::user_password
            .filter(user_password::dsl::user_id.eq(user.id))
            .get_result(db)
            .map_err(|err| anyhow!("Failed to get user password: {err}"))?;
        Ok(r)
    }
}

#[derive(Insertable)]
#[diesel(table_name = user_password)]
pub struct UserPasswordNew {
    pub user_id: i64,
    pub last_updated_date: chrono::DateTime<chrono::Utc>,
    password_hash: String,
}

impl UserPasswordNew {
    pub fn new(
        db: &mut diesel::PgConnection,
        user: &user_core::db::user::User,
        password: &SecStr,
    ) -> Result<UserPassword> {
        let password_hash = hash_password(password)?;
        let r = diesel::insert_into(test_db::schema::user_password::dsl::user_password)
            .values(&Self {
                user_id: user.id,
                last_updated_date: chrono::Utc::now(),
                password_hash,
            })
            .get_result(db)
            .map_err(|err| anyhow!("Failed to add user password: {err}"))?;
        Ok(r)
    }
}
