use itertools::Itertools;

use crate::package::Binaries;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalizedBinary {
    pub name: String,
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
    /// let names_and_binaries = vec![
    ///     (&"has-no-binary", &Binaries::None),
    ///     (&"has-unnamed-binary", &Binaries::Named("cli.js".to_owned())),
    ///     (&"has-named-binaries", &Binaries::Unnamed(HashMap::from([
    ///         ("a".to_owned(), "bin/a.js".to_owned()),
    ///         ("b".to_owned(), "bin/b.js".to_owned())
    ///     ]))),
    /// ];
    ///
    /// let expected_output = vec![
    ///     NormalizedBinary {
    ///         name: "has-unnamed-binary".to_owned(),
    ///         location: "../has-unnamed-binary/cli.js".to_owned(),
    ///     },
    ///     NormalizedBinary {
    ///         name: "a".to_owned(),
    ///         location: "../has-named-binaries/bin/a".to_owned(),
    ///     },
    ///     NormalizedBinary {
    ///         name: "b".to_owned(),
    ///         location: "../has-named-binaries/bin/b".to_owned(),
    ///     },
    /// ];
    ///
    /// assert_eq!(NormalizedBinary::normalize_binaries(names_and_binaries), expected_output);
    /// ```
    pub fn normalize_binaries<'a>(
        binaries: Vec<(&'a String, &'a Binaries)>,
    ) -> Vec<NormalizedBinary> {
        binaries
            .into_iter()
            .flat_map(|(pkg_name, bin)| match bin {
                Binaries::None => Vec::default(),
                Binaries::Unnamed(location) => vec![NormalizedBinary {
                    name: pkg_name.clone(),
                    location: format!("../{}", location),
                }],
                Binaries::Named(map) => map
                    .iter()
                    .map(|(bin_name, in_pkg_location)| NormalizedBinary {
                        name: bin_name.clone(),
                        location: format!("../{}/{}", pkg_name, in_pkg_location),
                    })
                    .collect(),
            })
            .sorted()
            .collect()
    }
}
