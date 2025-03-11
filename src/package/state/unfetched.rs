use serde::{Deserialize, Serialize};

use crate::package::MetaData;

use super::State;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Unfetched {
    pub meta: MetaData,
}

impl State for Unfetched {}
