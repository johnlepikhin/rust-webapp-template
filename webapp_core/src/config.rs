use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct OpenAPI {
    /// URI which should respond with autogenerated OpenAPI specification
    pub spec_uri: String,
    /// URI which should respond with Swagger interface to OpenAPI specification
    pub swagger_uri: String,
}

impl Default for OpenAPI {
    fn default() -> Self {
        Self {
            spec_uri: "/doc/openapi.json".to_owned(),
            swagger_uri: "/doc/swagger".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct CORS {
    /// Allowed origins
    pub origins: Vec<webapp_yaml_config::url::Url>,
}

#[derive(Serialize, Deserialize, StructDoc, Clone)]
pub struct Config {
    /// Bind web application to specified address. For example, "127.0.0.1"
    pub bind_address: String,
    /// Bind web application to specified port. For example, 8080
    pub bind_port: u16,
    /// Keep-alive HTTP timeout
    #[serde(with = "humantime_serde")]
    pub keep_alive: std::time::Duration,
    /// Shutdown timeout for active HTTP connections
    #[serde(with = "humantime_serde")]
    pub shutdown_timeout: std::time::Duration,
    /// Logging configuration
    pub logger: crate::logging::Logger,
    /// Enable OpenAPI/Swagger documentation for HTTP API
    pub openapi: Option<OpenAPI>,
    /// CORS configuration
    pub cors: Option<CORS>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_owned(),
            bind_port: 8080,
            keep_alive: std::time::Duration::from_secs(75),
            shutdown_timeout: std::time::Duration::from_secs(5),
            logger: Default::default(),
            openapi: Some(OpenAPI::default()),
            cors: None,
        }
    }
}
