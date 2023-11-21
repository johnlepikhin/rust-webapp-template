use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecUtf8(secstr::SecUtf8);

impl From<secstr::SecUtf8> for SecUtf8 {
    fn from(v: secstr::SecUtf8) -> Self {
        Self(v)
    }
}

impl Deref for SecUtf8 {
    type Target = secstr::SecUtf8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SecUtf8> for secstr::SecUtf8 {
    fn from(val: SecUtf8) -> Self {
        val.0
    }
}

impl<'__s> utoipa::ToSchema<'__s> for SecUtf8 {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            "webapp_core.secstr.SecUtf8",
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::schema::SchemaType::String)
                .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Password)))
                .title(Some("Secret UTF-8 string"))
                .description(Some("This value should be kept in secret. On the server side, it will be hidden from logs"))
                .into(),
        )
    }
}
