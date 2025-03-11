use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Binaries;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MetaData {
    pub peer_dependencies: HashMap<String, String>,
    pub optional_peers: Vec<String>,
    pub binaries: Binaries,
}
