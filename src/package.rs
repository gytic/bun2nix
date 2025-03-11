use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use async_process::Command;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use state::State;
use store_prefetch::StorePrefetch;

use crate::error::{Error, Result};

mod binaries;
mod dump_nix_expression;
mod metadata;
mod state;
mod store_prefetch;
mod visitor;

pub use binaries::Binaries;
pub use metadata::MetaData;
pub use state::{Fetched, Unfetched};
pub use visitor::PackageVisitor;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
/// # Lockfile Package
///
/// An individual package found in a bun lockfile
pub struct Package<D: State> {
    /// The name of the package, as found in the `./node_modules` directory or in an import
    /// statement
    pub name: String,

    /// The package's identifier string for fetching from npm
    pub npm_identifier: String,

    /// The state the package is currently in
    pub data: D,
}

impl Package<Unfetched> {
    /// # Package Constructor
    ///
    /// Produce a new instance of an unfetched package
    pub fn new(name: String, npm_identifier: String, meta: MetaData) -> Self {
        Self {
            name,
            npm_identifier,
            data: Unfetched { meta },
        }
    }

    /// # Fetch One
    ///
    /// Prefetch a single package from a url without interacting with the cache and produce a fetched package
    pub async fn fetch_one(self) -> Result<Package<Fetched>> {
        let url = self.to_npm_url()?;

        let output = Command::new("nix")
            .args(["store", "prefetch-file", "--json", &url])
            .output()
            .await?;

        if !output.status.success() {
            return Err(Error::PrefetchStderr(String::from_utf8(output.stderr)?));
        }

        let store_return: StorePrefetch = serde_json::from_slice(&output.stdout)?;

        assert_eq!(
            51,
            store_return.hash.len(),
            "hash was not 51 chars: {}",
            store_return.hash
        );
        assert!(store_return.hash.contains("sha256"));

        Ok(Package {
            name: self.name,
            npm_identifier: self.npm_identifier,
            data: Fetched {
                url,
                hash: store_return.hash,
                binaries: self.data.meta.binaries,
            },
        })
    }

    /// # NPM url converter
    ///
    /// Takes a package in the form:
    /// ```jsonc
    /// ["@alloc/quick-lru@5.2.0", "", {}, ""]
    /// ```
    ///
    /// And builds a prefetchable npm url like:
    /// ```bash
    /// https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz
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
}

impl Package<Fetched> {
    /// # Generate Binary Symlinks
    ///
    /// Produces a list of binary names and symlinks to their correct location in
    /// `node_modules`
    pub fn generate_binary_symlinks(&self) -> Vec<(String, String)> {
        match &self.data.binaries {
            Binaries::None => Vec::default(),
            Binaries::Unnamed(pathless_link) => {
                let link = format!("../{}/{}", self.name, pathless_link);

                vec![(self.name.clone(), link)]
            }
            Binaries::Named(bin_map) => bin_map
                .iter()
                .map(|(bin_name, pathless_link)| {
                    let link = format!("../{}/{}", self.name, pathless_link);

                    (bin_name.to_owned(), link)
                })
                .sorted()
                .collect(),
        }
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

impl<D: State> Eq for Package<D> {}

#[test]
fn test_to_npm_url() {
    let package = Package {
        name: "bun-types".to_owned(),
        npm_identifier: "bun-types@1.2.4".to_owned(),
        ..Default::default()
    };

    let out = package.to_npm_url().unwrap();

    assert!(out == "https://registry.npmjs.org/bun-types/-/bun-types-1.2.4.tgz");
}

#[test]
fn test_to_npm_url_with_namespace() {
    let package = Package {
        name: "@alloc/quick-lru".to_owned(),
        npm_identifier: "@alloc/quick-lru@5.2.0".to_owned(),
        ..Default::default()
    };

    let out = package.to_npm_url().unwrap();

    assert!(out == "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz");
}
