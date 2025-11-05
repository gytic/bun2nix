//! This module holds the implementation for data about a given nix fetcher type

use std::{fmt::Debug, hash::Hash};

use askama::Template;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Template, Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
/// # Package Fetcher
///
/// Nix-translated fetcher for a given package
pub enum Fetcher {
    /// A package which must be retrieved with nix's `pkgs.fetchurl`
    #[template(path = "fetchurl.nix_template")]
    FetchUrl {
        /// The url to fetch the package from
        url: String,
        /// The hash of the downloaded results
        /// This can be derived from the bun lockfile
        hash: String,
    },
    /// A package which must be retrieved with nix's `pkgs.fetchtarball`
    #[template(path = "fetchtarball.nix_template")]
    FetchTarball {
        /// The url to fetch the package from
        url: String,
        /// The hash of the downloaded results
        /// This can be derived from the bun lockfile
        hash: String,
    },
    /// A package can be a path copied to the store directly
    #[template(path = "copy-to-store.nix_template")]
    CopyToStore {
        /// The path from the root to copy to the store
        path: String,
    },
}

impl Fetcher {
    /// # From NPM Package Name
    ///
    /// Initialize a fetcher from an npm identifier and
    /// it's hash
    pub fn new_npm_package(ident: &str, hash: String) -> Result<Self> {
        let url = Self::to_npm_url(ident)?;

        Ok(Self::FetchUrl { url, hash })
    }

    /// # From NPM Url
    ///
    /// Initialize a fetcher from an npm url and
    /// it's hash
    pub fn new_tarball_package(url: String, hash: String) -> Self {
        Self::FetchTarball { url, hash }
    }

    /// # NPM url converter
    ///
    /// Produce a url needed to fetch from the npm api from a package
    ///
    /// ## Usage
    ///```rust
    /// use bun2nix::package::Fetcher;
    ///
    /// let npm_identifier = "@alloc/quick-lru@5.2.0";
    ///
    /// assert_eq!(
    ///     Fetcher::to_npm_url(npm_identifier).unwrap(),
    ///     "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz"
    /// );
    /// ```
    pub fn to_npm_url(ident: &str) -> Result<String> {
        let Some((user, name_and_ver)) = ident.split_once("/") else {
            let Some((name, ver)) = ident.split_once("@") else {
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
}
