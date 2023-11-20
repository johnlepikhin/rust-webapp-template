use actix_web::get;

/// This is sample main page
#[utoipa::path(
    responses(
        (status = 200, description = "Sample main page"),
    ),
    tag = "HTML pages",
    )]
#[get("/index.html")]
pub async fn index(
    config: actix_web::web::Data<webapp_yaml_config::yaml::Config<crate::Config>>,
) -> &'static str {
    slog_scope::info!("plugin secret is {:?}", config.config.secret);

    "Hello world!"
}
