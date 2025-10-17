//! This module holds the implementation for data about a given nix fetcher type

use std::{fmt::Debug, hash::Hash};

use askama::Template;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(
    Template, Default, Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd, Hash,
)]
/// # Package Fetcher
///
/// Nix-translated fetcher for a given package
pub enum Fetcher {
    #[template(path = "fetchurl.nix_template")]
    FetchUrl { url: String, hash: String },
    #[template(source = "", ext = "nix_template")]
    // #[template(path = "copy-to-store.nix_template")]
    CopyToStore { path: String },
    #[default]
    #[template(source = "", ext = "nix_template")]
    Unknown,
}

impl Fetcher {
    pub fn new_npm_package(npm_identifier_raw: &str, hash: String) -> Result<Self> {
        Ok(Self::FetchUrl {
            url: Self::to_npm_url(&npm_identifier_raw)?,
            hash,
        })
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
