use actix_web::web::Data;
use anyhow::anyhow;
use diesel::prelude::*;
use test_db::schema::{user, user_session};
use webapp_core::SESSION_COOKIE_NAME;

pub struct User {
    pub user: crate::db::user::User,
    pub session: crate::db::user_session::UserSession,
}

impl User {
    pub fn logout(&self, db: &mut diesel::PgConnection) -> anyhow::Result<()> {
        diesel::delete(user_session::dsl::user_session.find(self.session.id)).execute(db)?;
        Ok(())
    }
}

impl actix_web::FromRequest for User {
    type Error = actix_web::Error;

    type Future = std::pin::Pin<Box<dyn futures_util::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok().map(|v| v.to_owned()))
            .or_else(|| {
                req.cookie(SESSION_COOKIE_NAME)
                    .map(|v| v.value().to_owned())
            });
        let token = match token {
            Some(v) => v,
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorForbidden(
                        "Authorization header nor session cookie are not provided",
                    ))
                })
            }
        };
        let db = match req.app_data::<Data<test_db::db::DB>>() {
            Some(v) => v.clone(),
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorInternalServerError(
                        "No user DB available",
                    ))
                })
            }
        };

        let client_ip = match req.peer_addr() {
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorForbidden(
                        "No user peer address available",
                    ))
                })
            }
            Some(v) => ipnet::IpNet::from(v.ip()),
        };

        let now = chrono::Utc::now();

        Box::pin(async move {
            db.pool
                .with_transaction(move |conn| {
                    let session: crate::db::user_session::UserSession = diesel::update(
                        user_session::dsl::user_session.filter(user_session::dsl::token.eq(token)),
                    )
                    .set((
                        user_session::dsl::last_seen_date.eq(now),
                        user_session::dsl::last_address.eq(client_ip),
                        user_session::dsl::requests_count.eq(user_session::dsl::requests_count + 1),
                    ))
                    .get_result(conn)?;

                    let user_id = match session.user_id {
                        None => return Err(anyhow!("Not authorized")),
                        Some(v) => v,
                    };

                    let user: crate::db::user::User = diesel::update(user::dsl::user.find(user_id))
                        .set(user::dsl::last_seen_date.eq(Some(now)))
                        .get_result::<crate::db::user::User>(conn)?;

                    Ok(User { user, session })
                })
                .await
                .map_err(|_| actix_web::error::ErrorForbidden("Not authorized"))
        })
    }
}
