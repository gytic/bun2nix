use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// # Prefetch Output
///
/// A model of the results returned by `nix flake prefetch <url> --json`
pub struct PrefetchOutput {
    hash: String,
    locked: Lock,
    original: Original,
    store_path: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    last_modified: u32,
    nar_hash: String,
    #[serde(rename = "type")]
    flake_type: String,
    url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    #[serde(rename = "type")]
    flake_type: String,
    url: String,
}

