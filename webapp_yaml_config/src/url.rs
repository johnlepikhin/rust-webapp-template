use serde::{Deserialize, Serialize};
use std::ops::Deref;
use structdoc::Documentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url(url::Url);

impl structdoc::StructDoc for Url {
    fn document() -> Documentation {
        Documentation::leaf("URL string")
    }
}

impl From<url::Url> for Url {
    fn from(v: url::Url) -> Self {
        Self(v)
    }
}

impl Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
