use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    ToSchema,
};

pub fn new() -> utoipa::openapi::OpenApi {
    let (secstr_name, secstr) = crate::secstr::SecUtf8::schema();
    let mut components = utoipa::openapi::ComponentsBuilder::new()
        .schema(secstr_name, secstr)
        .build();
    components.add_security_scheme(
        "session_cookie",
        SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(crate::SESSION_COOKIE_NAME))),
    );
    components.add_security_scheme(
        "authorization_header",
        SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
    );

    utoipa::openapi::OpenApiBuilder::new()
        .info(
            utoipa::openapi::InfoBuilder::new()
                .title("application name")
                .description(Some("application description"))
                .license(Some(utoipa::openapi::License::new("MIT")))
                .contact(Some(
                    utoipa::openapi::ContactBuilder::new()
                        .name(Some("author name"))
                        .email(Some("author email"))
                        .build(),
                ))
                .version(env!("CARGO_PKG_VERSION"))
                .build(),
        )
        .paths(utoipa::openapi::path::Paths::new())
        .components(Some(components))
        .build()
}
