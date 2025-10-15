use std::{fmt::Debug, hash::Hash};

use rinja::Template;
use serde::{Deserialize, Serialize};

// # Copy To Store Fetcher
//
// This represents a path relative to the
// `src` root that gets copied into the
// store
#[derive(Template)]
#[template(path = "copy-to-store.jinja")]
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct CopyToStore {
    pub path: String,
}
