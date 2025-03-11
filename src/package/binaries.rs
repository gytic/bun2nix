use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Binaries {
    #[default]
    None,
    Unnamed(String),
    Named(HashMap<String, String>),
}

impl TryFrom<String> for Binaries {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(serde_json::from_str(&value)?)
    }
}
