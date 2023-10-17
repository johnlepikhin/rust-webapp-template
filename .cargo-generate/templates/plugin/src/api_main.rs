use actix_web::Responder;
use paperclip::actix::api_v2_operation;

/// This is sample main page
#[api_v2_operation]
pub async fn index(
    config: paperclip_actix::web::Data<webapp_yaml_config::yaml::Config<crate::Config>>,
) -> impl Responder {
    let my_secret = config
        .with_config(|config| Ok(config.secret.clone()))
        .unwrap();
    slog_scope::info!("plugin secret is {:?}", my_secret);

    "Hello world!"
}
