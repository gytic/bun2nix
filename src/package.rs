//! This module holds the core implementation for the package type and related methods

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

mod fetcher;

pub use fetcher::Fetcher;
use wyhash::final3::wyhash;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
/// # Package
///
/// An individual package found in a bun lockfile.
pub struct Package {
    /// The name of the package, as found in the `./node_modules` directory or in an import
    /// statement
    pub name: String,

    /// The fetch method to use for the package
    pub fetcher: Fetcher,
}

impl Package {
    pub fn from_raw_npm_identifier(name: String, fetcher: Fetcher) -> Self {
        let npm_identifier_file_safe = name.replace("/", "+");

        Self::from_file_safe_npm_identifier(npm_identifier_file_safe, fetcher)
    }

    pub fn from_file_safe_npm_identifier(name: String, fetcher: Fetcher) -> Self {
        // assert!(
        //     !name.contains("/"),
        //     "File safe npm identifier cannot contain a `/` character, please use the from raw method instead"
        // );

        Self { name, fetcher }
    }

    pub fn from_workspace_identifier(name: String, fetcher: Fetcher) -> Self {
        Self { name, fetcher }
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.fetcher.hash(state);
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fetcher == other.fetcher
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Package {}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.name, &self.fetcher).cmp(&(&other.name, &other.fetcher))
    }
}

const WYHASH_SEED: u64 = 0;

const WYHASH_SECRET: [u64; 4] = [
    0xa0761d6478bd642f,
    0xe7037ed1a0b428db,
    0x8ebc6af09c88c6e3,
    0x589965cc75374cc3,
];

/// Produce a correct bun cache folder name for a given npm identifier
///
/// Adapted from [here](https://github.com/oven-sh/bun/blob/134341d2b48168cbb86f74879bf6c1c8e24b799c/src/install/PackageManager/PackageManagerDirectories.zig#L288)
///
///
/// This takes a string with components like:
/// ```block
/// react@1.2.3-beta.1+build.123
///             ^^^^^^ ^^^^^^^^^
///             pre    build
/// ```
/// and hashes them with the wyhash algorithm to produce:
///
/// ```block
/// react@1.2.3-a1b2c3d4+E5F6G7H8
/// ```
///
/// ## Usage
///
///```rust
/// use bun2nix::package::cached_npm_package_folder_print_basename;
///
/// let a = "react@1.2.3-beta.1+build.123".to_string();
/// let b = "tailwindcss@4.0.0-beta.9".to_string();
/// let c = "react@1.2.3+build.123".to_string();
/// let d = "react@1.2.3".to_string();
///
/// //assert_eq!(
/// //    cached_npm_package_folder_print_basename(a),
/// //    "react@1.2.3-a1b2c3d4+E5F6G7H8"
/// //);
/// assert_eq!(
///     cached_npm_package_folder_print_basename(b),
///     "tailwindcss@4.0.0-73c5c46324e78b9b"
/// );
/// assert_eq!(
///     cached_npm_package_folder_print_basename(c),
///     "react@1.2.3+E5F6G7H8"
/// );
/// assert_eq!(
///     cached_npm_package_folder_print_basename(d),
///     "react@1.2.3"
/// );
/// ```
pub fn cached_npm_package_folder_print_basename(pkg: String) -> String {
    if let Some((name_and_ver, pre_and_build)) = pkg.split_once("-") {
        if let Some((pre, build)) = pkg.split_once("+") {
            return format!(
                "{}-{:x}+{:X}",
                name_and_ver,
                wyhash(pre.as_bytes(), WYHASH_SEED, WYHASH_SECRET),
                wyhash(build.as_bytes(), WYHASH_SEED, WYHASH_SECRET),
            );
        };

        println!("hashing: '{}'", pre_and_build); // "beta.9"

        return format!(
            "{}-{:x}",
            name_and_ver,
            wyhash(pre_and_build.as_bytes(), WYHASH_SEED, WYHASH_SECRET)
        );
    };

    if let Some((name_and_ver, build)) = pkg.split_once("+") {
        return format!(
            "{}+{:X}",
            name_and_ver,
            wyhash(build.as_bytes(), WYHASH_SEED, WYHASH_SECRET)
        );
    };

    return pkg;
}
