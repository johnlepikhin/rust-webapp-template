use actix_http::StatusCode;
use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Data required for login
#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct LoginRequest {
    /// User login
    username: String,
    /// User password
    password: webapp_core::secstr::SecUtf8,
}

/// Response for login
#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    /// Session token
    token: webapp_core::secstr::SecUtf8,
}

/// Creates new session for user
#[utoipa::path(
    responses(
        (status = OK, description = "Session created", body = LoginResponse),
        (status = FORBIDDEN, description = "No such user or password is incorrect")
    ),
    tag = "User",
    )]
#[post("/api/v1/user/login/password")]
pub async fn login(
    db: Data<test_db::db::DB>,
    login_request: web::Json<LoginRequest>,
    req: HttpRequest,
) -> actix_web::error::Result<impl Responder> {
    let client_ip = match req.peer_addr() {
        None => {
            slog_scope::error!("No peer address available");
            actix_web::error::Result::Err(actix_web::error::InternalError::new(
                "No such user or password is incorrect",
                StatusCode::FORBIDDEN,
            ))?
        }
        Some(v) => v,
    };
    let login_request_transaction = login_request.clone();
    let user_token = db
        .pool
        .with_transaction(move |conn| {
            let user =
                user_core::db::user::User::of_username(conn, &login_request_transaction.username)?;
            user.logged_in(conn)?;
            crate::db::user_password::UserPassword::of_user(conn, &user)?.validate_password(
                &database_pg::secstr::SecUtf8::from(Into::<secstr::SecUtf8>::into(
                    login_request_transaction.password,
                )),
            )?;
            user_core::db::user_session::UserSession::new(conn, &user, client_ip.ip().into())
        })
        .await
        .map_err(|err| {
            slog_scope::warn!("Login failed for request {:?}: {err}", login_request);
            actix_web::error::InternalError::new(
                "No such user or password is incorrect",
                StatusCode::FORBIDDEN,
            )
        })?;

    Ok(web::Json(LoginResponse {
        token: webapp_core::secstr::SecUtf8::from(Into::<secstr::SecUtf8>::into(user_token.token)),
    }))
}
