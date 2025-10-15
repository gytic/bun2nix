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
    #[template(path = "copy-to-store.nix_template")]
    CopyToStore { path: String },
    #[default]
    #[template(source = "", ext = "nix_template")]
    Unknown,
}

impl Fetcher {
    pub fn from_raw_npm_identifier(ident: String, hash: String) -> Result<Self> {
        let npm_identifier_file_safe = ident.replace("/", "+");

        Self::from_file_safe_npm_identifier(npm_identifier_file_safe, hash)
    }

    pub fn from_file_safe_npm_identifier(ident: String, hash: String) -> Result<Self> {
        assert!(
            !ident.contains("/"),
            "File safe npm identifier cannot contain a `/` character, please use the from raw method instead"
        );

        Ok(Self::FetchUrl {
            url: Self::to_npm_url(&ident)?,
            hash,
        })
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
