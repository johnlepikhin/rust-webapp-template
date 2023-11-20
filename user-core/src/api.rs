use actix_http::StatusCode;
use actix_web::{cookie::Cookie, post, web::Data, HttpResponse, HttpResponseBuilder};
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
#[post("/api/v1/user/session/logout")]
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
