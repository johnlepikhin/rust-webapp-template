use actix_http::StatusCode;
use actix_web::{cookie::Cookie, get, post, web::Data, HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use utoipa::ToSchema;
use webapp_core::SESSION_COOKIE_NAME;

/// Deletes current session of user
#[utoipa::path(
    responses(
        (status = OK, description = "Session removed successfully"),
        (status = FORBIDDEN, description = "User not logged in")
    ),
    security(("session_cookie" = []), ("authorization_header" = [])),
    tag = "User",
    )]
#[post("/api/v1/user_session/logout")]
pub async fn logout(
    user: crate::user::User,
    db: Data<test_db::db::DB>,
) -> actix_web::error::Result<HttpResponse> {
    db.pool
        .with_transaction(move |conn| user.logout(conn))
        .await
        .map_err(|err| {
            actix_web::error::InternalError::new(err.to_string(), StatusCode::FORBIDDEN)
        })?;

    let mut resp = HttpResponseBuilder::new(StatusCode::OK).body("Logged out");
    resp.add_removal_cookie(&Cookie::build(SESSION_COOKIE_NAME, "").finish())?;

    Ok(resp)
}

/// Element of user list
#[derive(Serialize, ToSchema)]
pub struct UserListUser {
    /// Internal user ID
    pub id: i64,
    /// When user was created
    pub create_date: chrono::DateTime<chrono::Utc>,
    /// When authenticated user used API last time
    pub last_seen_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Total number of times user ever logged in
    pub login_count: i64,
    /// Username
    pub username: String,
    /// User person
    pub person: String,
}

/// Gets list of users
#[utoipa::path(
    responses(
        (status = OK, description = "User list", body = [UserListUser], content_type = "application/json",
         headers(
             ("X-Total-Count" = usize, description = "Total count of users"),
         ))),
    tag = "User",
    )]
#[get("/api/v1/user")]
async fn user_list(
    db: Data<test_db::db::DB>,
) -> actix_web::error::Result<webapp_core::response::APIList<UserListUser>> {
    let (list, count) = db
        .pool
        .with_transaction(move |conn| {
            use database_pg::paginate::*;
            use diesel::prelude::*;
            use test_db::schema::user;

            let r = user::table
                .select(user::all_columns)
                .paginate(1, 2)
                .load_and_count_pages::<crate::db::user::User>(conn)?;
            Ok(r)
        })
        .await
        .map_err(|err| {
            actix_web::error::InternalError::new(err.to_string(), StatusCode::FORBIDDEN)
        })?;

    let list = list
        .into_iter()
        .map(|v| UserListUser {
            id: v.id,
            create_date: v.create_date,
            last_seen_date: v.last_seen_date,
            login_count: v.login_count,
            username: v.username,
            person: v.person,
        })
        .collect();

    webapp_core::response::APIList::ok(list, count)
}
