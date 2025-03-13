use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
/// # Package Binaries
///
/// A model of the executable binaries that should be created for each package
pub enum Binaries {
    #[default]
    /// No binaries should be produced for this package
    None,
    /// Produce a single binary, with a name that matches the name of the package, pointing to this
    /// location
    Unnamed(String),
    /// Map of binary names to binary locations
    Named(HashMap<String, String>),
}

impl TryFrom<String> for Binaries {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(serde_json::from_str(&value)?)
    }
}
