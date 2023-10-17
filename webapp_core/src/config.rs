use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct OpenAPI {
    /// URI which should respond with autogenerated OpenAPI specification
    pub spec_uri: String,
    /// URI which should respond with Swagger interface to OpenAPI specification
    pub swagger_uri: Option<String>,
}

impl Default for OpenAPI {
    fn default() -> Self {
        Self {
            spec_uri: "/doc/openapi.json".to_owned(),
            swagger_uri: Some("/doc/swagger".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct Config {
    /// Bind web application to specified address. For example, "127.0.0.1"
    pub bind_address: String,
    /// Bind web application to specified port. For example, 8080
    pub bind_port: u16,
    /// Logging configuration
    pub loggers: crate::logging::Loggers,
    /// Enable OpenAPI/Swagger documentation for HTTP API
    pub openapi: Option<OpenAPI>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_owned(),
            bind_port: 8080,
            loggers: Default::default(),
            openapi: Some(OpenAPI::default()),
        }
    }
}
