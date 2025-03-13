use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorePrefetch {
    pub hash: String,
    pub store_path: String,
}
