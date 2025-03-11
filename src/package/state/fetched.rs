use serde::{Deserialize, Serialize};

use crate::package::Binaries;

use super::State;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fetched {
    /// The url to fetch the package from
    pub url: String,

    /// The prefetched hash of the package
    pub hash: String,

    /// Binaries to create symlinks for
    pub binaries: Binaries,
}

impl State for Fetched {}
