use serde::{Deserialize, Serialize};
use std::ops::Deref;
use structdoc::Documentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecUtf8(secstr::SecUtf8);

impl structdoc::StructDoc for SecUtf8 {
    fn document() -> Documentation {
        Documentation::leaf("Secret string")
    }
}

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
