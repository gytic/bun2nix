use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use itertools::Itertools;

use crate::package::Binaries;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// # Normalized Binary
///
/// Normal version of a binary symlink with a name pointing to a location
pub struct NormalizedBinary {
    /// The file name to create the symlink under
    pub name: String,

    /// The actual file to point the symlink to
    pub location: String,
}

impl NormalizedBinary {
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
    /// let unnamed = Binaries::Unnamed("cli.js".to_owned());
    /// let named = Binaries::Named(HashMap::from([
    ///     ("a".to_owned(), "bin/a.js".to_owned()),
    ///     ("b".to_owned(), "bin/b.js".to_owned())
    /// ]));
    ///
    /// let names_and_binaries = vec![
    ///     ("has-no-binary", &none),
    ///     ("has-unnamed-binary", &unnamed),
    ///     ("has-named-binaries", &named),
    /// ];
    ///
    /// let expected_output = vec![
    ///     NormalizedBinary {
    ///         name: "a".to_owned(),
    ///         location: "../has-named-binaries/bin/a.js".to_owned(),
    ///     },
    ///     NormalizedBinary {
    ///         name: "b".to_owned(),
    ///         location: "../has-named-binaries/bin/b.js".to_owned(),
    ///     },
    ///     NormalizedBinary {
    ///         name: "has-unnamed-binary".to_owned(),
    ///         location: "../has-unnamed-binary/cli.js".to_owned(),
    ///     },
    /// ];
    ///
    /// let mut actual = NormalizedBinary::normalize_binaries(names_and_binaries);
    ///
    /// actual.sort();
    ///
    /// assert_eq!(actual, expected_output);
    /// ```
    pub fn normalize_binaries<'a>(binaries: Vec<(&'a str, &'a Binaries)>) -> Vec<NormalizedBinary> {
        binaries
            .into_iter()
            .flat_map(|(pkg_name, bin)| match bin {
                Binaries::None => Vec::default(),
                Binaries::Unnamed(location) => {
                    let name_path = PathBuf::from(pkg_name);

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
                    .iter()
                    .map(|(bin_name, in_pkg_location)| NormalizedBinary {
                        name: bin_name.clone(),
                        location: format!("../{}/{}", pkg_name, in_pkg_location),
                    })
                    .collect(),
            })
            .sorted()
            .dedup_by(|a, b| a.name == b.name)
            .collect()
    }
}

impl Hash for NormalizedBinary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
