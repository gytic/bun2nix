use serde::{Deserialize, Serialize};

use super::State;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
/// # Fetched Package Data
///
/// Data held by a fetched package
pub struct Fetched {
    /// The url to fetch the package from
    pub url: String,

    /// The prefetched hash of the package's files
    pub hash: String,
}

impl State for Fetched {}
