use serde::{Deserialize, Serialize};
use structdoc::StructDoc;

#[derive(Serialize, Deserialize, StructDoc)]
pub struct Config {
    /// Minimum password length
    pub min_password_length: usize,
}
