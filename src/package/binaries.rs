use std::{collections::HashMap, path::Path};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    nix_expression::NormalizedBinary,
};

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

impl Binaries {
    /// # Normalize Binaries
    ///
    /// Turn a set of binaries into a flat vector of name to location symlink struct
    /// representations
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use bun2nix::{package::Binaries, nix_expression::NormalizedBinary};
    /// use std::collections::{HashMap, HashSet};
    ///
    /// let none = Binaries::None;
    ///
    /// assert_eq!(none.normalize("has-no-binary"), Vec::default());
    ///
    /// let unnamed = Binaries::Unnamed("cli.js".to_owned());
    ///
    /// let expected = vec![
    ///     NormalizedBinary {
    ///         name: "has-unnamed-binary".to_owned(),
    ///         location: "../has-unnamed-binary/cli.js".to_owned(),
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     unnamed.normalize("has-unnamed-binary"),
    ///     expected
    /// );
    ///
    /// let named = Binaries::Named(HashMap::from([
    ///     ("a".to_owned(), "bin/a.js".to_owned()),
    ///     ("b".to_owned(), "bin/b.js".to_owned())
    /// ]));
    ///
    /// let expected = vec![
    ///     NormalizedBinary {
    ///         name: "a".to_owned(),
    ///         location: "../has-named-binaries/bin/a.js".to_owned(),
    ///     },
    ///     NormalizedBinary {
    ///         name: "b".to_owned(),
    ///         location: "../has-named-binaries/bin/b.js".to_owned(),
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     named.normalize("has-named-binaries"),
    ///     expected
    /// );
    /// ```
    pub fn normalize(self, pkg_name: &str) -> Vec<NormalizedBinary> {
        match self {
            Binaries::None => Vec::default(),
            Binaries::Unnamed(location) => {
                let name_path = Path::new(&pkg_name);

                let out_name = name_path
                    .components()
                    .last()
                    .map(|x| x.as_os_str().to_string_lossy());

                vec![NormalizedBinary {
                    name: out_name.unwrap_or_default().to_string(),
                    location: format!("../{}/{}", pkg_name, location),
                }]
            }
            Binaries::Named(map) => map
                .into_iter()
                .map(|(bin_name, in_pkg_location)| NormalizedBinary {
                    name: bin_name,
                    location: format!("../{}/{}", pkg_name, in_pkg_location),
                })
                .collect(),
        }
        .into_iter()
        .sorted()
        .dedup_by(|a, b| a.name == b.name)
        .collect()
    }
}

impl TryFrom<String> for Binaries {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(serde_json::from_str(&value)?)
    }
}
