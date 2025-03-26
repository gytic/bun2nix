use serde::{Deserialize, Serialize};

use crate::nix_expression::NormalizedBinary;

use super::State;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
/// # Normalized Package Data
///
/// Data held by a fetched package
pub struct Normalized {
    /// The path to write out at
    pub out_path: String,

    /// The url to fetch the package from
    pub url: String,

    /// The prefetched hash of the package's files
    pub hash: String,

    /// Binaries to create symlinks for
    pub binaries: Vec<NormalizedBinary>,
}

impl Normalized {
    /// # Normalize Name to Path
    ///
    /// Converts a regular package name into it's output path
    ///
    /// ```rust
    /// use bun2nix::package::Normalized;
    /// use std::path::Path;
    ///
    /// let simple = Normalized::convert_name_to_out_path("a");
    ///
    /// assert_eq!(&simple, "a");
    ///
    /// let namespaced = Normalized::convert_name_to_out_path("@types/a");
    ///
    /// assert_eq!(&namespaced, "@types/a");
    ///
    /// let subpackage = Normalized::convert_name_to_out_path("@types/a/b");
    ///
    /// assert_eq!(&subpackage, "@types/a/node_modules/b");
    /// ```
    pub fn convert_name_to_out_path(name: &str) -> String {
        let has_at = name.contains("@");
        let mut sections = name.split("/").map(|s| s.to_string()).collect::<Vec<_>>();

        match (sections.len(), has_at) {
            (0, _) | (1, true) => name.to_string(),
            (_, false) => sections.join("/node_modules/"),
            (_, true) => {
                let start = sections.drain(0..1).collect::<Vec<_>>().join("/");

                let rest = sections.join("/node_modules/");

                format!("{}/{}", start, rest)
            }
        }
    }
}

impl State for Normalized {}
