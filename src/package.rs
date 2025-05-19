//! This module holds the core implementation for the package type and related methods

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};
use state::State;

use crate::error::{Error, Result};

mod binaries;
mod metadata;
mod normalized_binary;
mod state;

pub use binaries::Binaries;
pub use metadata::MetaData;
pub use normalized_binary::NormalizedBinary;
pub use state::{Extracted, Normalized};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
/// # Package
///
/// An individual package found in a bun lockfile.
pub struct Package<D: State> {
    /// The prefetched package hash
    pub hash: String,

    /// The name of the package, as found in the `./node_modules` directory or in an import
    /// statement
    pub name: String,

    /// The package's identifier string for fetching from npm
    pub npm_identifier: String,

    /// The state the package is currently in
    pub data: D,
}

impl Package<Extracted> {
    /// # Package Constructor
    ///
    /// Produce a new instance of a just extracted package
    pub fn new(name: String, npm_identifier: String, hash: String, binaries: Binaries) -> Self {
        Self {
            name,
            npm_identifier,
            hash,
            data: Extracted { binaries },
        }
    }

    /// # NPM url converter
    ///
    /// Produce a url needed to fetch from the npm api from a package
    ///
    /// ## Usage
    ///```rust
    /// use bun2nix::Package;
    ///
    /// let package = Package {
    ///     npm_identifier: "@alloc/quick-lru@5.2.0".to_owned(),
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(package.to_npm_url().unwrap(), "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz")
    /// ```
    pub fn to_npm_url(&self) -> Result<String> {
        let Some((user, name_and_ver)) = self.npm_identifier.split_once("/") else {
            let Some((name, ver)) = self.npm_identifier.split_once("@") else {
                return Err(Error::NoAtInPackageIdentifier);
            };

            return Ok(format!(
                "https://registry.npmjs.org/{}/-/{}-{}.tgz",
                name, name, ver
            ));
        };

        let Some((name, ver)) = name_and_ver.split_once("@") else {
            return Err(Error::NoAtInPackageIdentifier);
        };

        Ok(format!(
            "https://registry.npmjs.org/{}/{}/-/{}-{}.tgz",
            user, name, name, ver
        ))
    }

    /// # Normalize Packages
    ///
    /// Normalizes a package's data fields to prepare it to be output
    ///
    /// This includes building the output path in `node_modules` and a proper binaries list
    pub fn normalize(self) -> Result<Package<Normalized>> {
        Ok(Package {
            data: Normalized {
                out_path: Normalized::convert_name_to_out_path(&self.name),
                url: self.to_npm_url()?,
                binaries: self.data.binaries.normalize(&self.name),
            },
            npm_identifier: self.npm_identifier,
            hash: self.hash,
            name: self.name,
        })
    }
}

impl<D: State> Hash for Package<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.npm_identifier.hash(state);
    }
}

impl<D: State> PartialEq for Package<D> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.npm_identifier == other.npm_identifier
    }
}

impl<D: State> PartialOrd for Package<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<D: State> Eq for Package<D> {}

impl<D: State> Ord for Package<D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.name, &self.npm_identifier).cmp(&(&other.name, &other.npm_identifier))
    }
}
