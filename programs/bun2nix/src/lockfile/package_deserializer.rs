use crate::{package::Fetcher, Package};

use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

type Values = Vec<serde_json::Value>;

/// # Package Deserializer
///
/// Deserializes a given bun lockfile entry line into it's
/// name and nix fetcher implementation
pub struct PackageDeserializer {
    /// The name for the package
    pub name: String,

    /// The list of serde json values for the tuple in question
    pub values: Values,
}

#[derive(Debug, Deserialize, Serialize)]
struct Prefetch {
    hash: String,
}

impl PackageDeserializer {
    /// # Deserialize package
    ///
    /// Deserialize a given package from it's lockfile representation
    pub fn deserialize_package(name: String, values: Values) -> Result<Package> {
        let arity = values.len();
        let deserializer = Self { name, values };

        match arity {
            1 => deserializer.deserialize_workspace_package(),
            2 => deserializer.deserialize_tarball_or_file_package(),
            4 => deserializer.deserialize_npm_package(),
            x => Err(Error::UnexpectedPackageEntryLength(x)),
        }
    }

    /// # Deserialize an NPM Package
    ///
    /// Deserialize an npm package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 4
    pub fn deserialize_npm_package(mut self) -> Result<Package> {
        let npm_identifier_raw = swap_remove_value(&mut self.values, 0);
        let hash = swap_remove_value(&mut self.values, 0);

        debug_assert!(
            hash.contains("sha512-"),
            "Expected hash to be in sri format and contain sha512"
        );

        let fetcher = Fetcher::new_npm_package(&npm_identifier_raw, hash)?;

        Ok(Package::new(npm_identifier_raw, fetcher))
    }

    /// # Deserialize a tarball or file package
    ///
    /// Deserialize a tarball or file package from it's bun
    /// lockfile representation
    ///
    /// These are grouped together as both lockfile
    /// representations are a tupe of arity 2, hence
    /// paths starting with `http` are considered
    /// tarballs
    pub fn deserialize_tarball_or_file_package(mut self) -> Result<Package> {
        let id = swap_remove_value(&mut self.values, 0);
        let Some(path) = Self::drain_after_substring(id.to_string(), "@") else {
            return Err(Error::NoAtInPackageIdentifier);
        };

        if path.starts_with("http") {
            Self::deserialize_tarball_package(path)
        } else {
            Self::deserialize_file_package(self.name, path)
        }
    }

    /// # Deserialize a file package
    ///
    /// Deserialize a file package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_file_package(name: String, path: String) -> Result<Package> {
        debug_assert!(
            !path.contains("http"),
            "File path can never contain http, because then it would be a tarball"
        );

        Ok(Package::new(name, Fetcher::CopyToStore { path }))
    }

    /// # Deserialize a tarball package
    ///
    /// Deserialize a tarball package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_tarball_package(url: String) -> Result<Package> {
        debug_assert!(url.contains("http"), "Expected tarball url to contain http");

        let cmd_res = Command::new("nix")
            .args(["flake", "prefetch", &url, "--json"])
            .output()
            .map_err(|err| Error::FetchingFailed(err.to_string()))?;

        let stdout = str::from_utf8(&cmd_res.stdout)
            .map_err(|err| Error::InvalidUtf8String(err.to_string()))?;

        let prefetch: Prefetch = serde_json::from_str(stdout)?;

        let name = format!("tarball:{}", url);
        let fetcher = Fetcher::new_tarball_package(url, prefetch.hash);

        Ok(Package::new(name, fetcher))
    }

    /// # Deserialize a workspace package
    ///
    /// Deserialize a workspace package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_workspace_package(mut self) -> Result<Package> {
        let id = swap_remove_value(&mut self.values, 0);
        let Some(path) = Self::drain_after_substring(id.to_string(), "workspace:") else {
            return Err(Error::MissingWorkspaceSpecifier);
        };

        Ok(Package::new(self.name, Fetcher::CopyToStore { path }))
    }

    fn drain_after_substring(mut input: String, sub: &str) -> Option<String> {
        let pos = input.rfind(sub)? + sub.len();

        Some(input.drain(pos..).collect())
    }
}

fn swap_remove_value(values: &mut Values, index: usize) -> String {
    let mut value = values.swap_remove(index).to_string();

    #[cfg(debug_assertions)]
    let mut chars = value.chars();

    debug_assert!(
        chars.next().unwrap() == '"',
        "Value should start with a quote"
    );
    debug_assert!(
        chars.last().unwrap() == '"',
        "Value should end with a quote"
    );

    value.drain(1..value.len() - 1).collect()
}
