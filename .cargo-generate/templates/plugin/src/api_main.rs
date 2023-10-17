use actix_web::Responder;
use paperclip::actix::api_v2_operation;

/// This is sample main page
#[api_v2_operation]
pub async fn index() -> impl Responder {
    "Hello world!"
}
