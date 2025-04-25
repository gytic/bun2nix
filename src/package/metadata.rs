use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Binaries;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
/// Package Meta Data
///
/// Extra information about a package, such as peer dependencies or binaries
pub struct MetaData {
    /// A map of peer depdendency names to their versions
    pub peer_dependencies: HashMap<String, String>,

    /// Optional peer dependencies
    pub optional_peers: Vec<String>,

    /// Package regular dependencies
    pub dependencies: HashMap<String, String>,

    /// Package binaries
    #[serde(rename = "bin")]
    pub binaries: Binaries,
}
