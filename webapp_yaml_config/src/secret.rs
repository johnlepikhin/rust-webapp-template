use serde::{Deserialize, Serialize};
use std::ops::Deref;
use structdoc::{Documentation, StructDoc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecUtf8String(secstr::SecUtf8);

impl structdoc::StructDoc for SecUtf8String {
    fn document() -> Documentation {
        Documentation::leaf("Secret string")
    }
}

impl From<secstr::SecUtf8> for SecUtf8String {
    fn from(v: secstr::SecUtf8) -> Self {
        Self(v)
    }
}

impl Deref for SecUtf8String {
    type Target = secstr::SecUtf8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, StructDoc)]
/// Secret keeper
pub enum Secret {
    /// Plaintext secret string
    String(SecUtf8String),
    /// Secret string from provided environment variable
    FromEnv(String),
    /// Secret string from provided command STDOUT
    FromCommand(String),
}

impl Secret {
    pub fn unsecure(&self) -> anyhow::Result<String> {
        match self {
            Self::String(v) => Ok(v.unsecure().to_owned()),
            Self::FromEnv(env_var) => {
                let v = std::env::var(env_var)?;
                Ok(v)
            }
            Self::FromCommand(cmd) => {
                let v = std::process::Command::new(cmd).output()?.stdout;
                let v = String::from_utf8(v)?;
                Ok(v)
            }
        }
    }
}
