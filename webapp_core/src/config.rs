use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc)]
pub struct Config {
    /// Bind web application to specified address. For example, "127.0.0.1"
    pub bind_address: String,
    /// Bind web application to specified port. For example, 8080
    pub bind_port: u16,
    /// Logging configuration
    pub loggers: crate::logging::Loggers,
}
