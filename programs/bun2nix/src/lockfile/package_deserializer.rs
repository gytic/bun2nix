use crate::{Package, package::Fetcher};

use std::process::Command;

use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, Visitor},
};

pub struct PackageDeserializer<'a> {
    pub name: String,
    pub values: Vec<serde_json::Value>,
    pub packages: &'a mut Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Prefetch {
    hash: String,
}

impl<'a> PackageDeserializer<'a> {
    /// # Deserialize an NPM Package
    ///
    /// Deserialize an npm package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 4
    pub fn deserialize_npm_package<E>(self) -> Result<(), E>
    where
        E: de::Error,
    {
        let npm_identifier_raw = self.values[0]
            .as_str()
            .ok_or_else(|| de::Error::custom("Invalid npm_identifier format"))?
            .to_string();

        let hash = self.values[3]
            .as_str()
            .ok_or_else(|| de::Error::custom("Invalid hash format"))?
            .to_string();

        assert!(
            hash.contains("sha512-"),
            "Expected hash to be in sri format and contain sha512"
        );

        let fetcher = Fetcher::new_npm_package(&npm_identifier_raw, hash).map_err(|_| {
            de::Error::custom("Failed to create npm url for npm package while deserializing")
        })?;

        let pkg = Package::new(npm_identifier_raw, fetcher);
        self.packages.push(pkg);

        Ok(())
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
    pub fn deserialize_tarball_or_file_package<E>(self) -> Result<(), E>
    where
        E: de::Error,
    {
        let Some(id) = self.values[0].as_str() else {
            return Ok(());
        };

        let Some(path) = Self::drain_after_substring(id.to_string(), "@") else {
            return Ok(());
        };

        if path.starts_with("http") {
            Self::deserialize_tarball_package(self.name, path, self.packages)
        } else {
            Self::deserialize_file_package(self.name, path, self.packages)
        }
    }

    /// # Deserialize a file package
    ///
    /// Deserialize a file package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_file_package<E>(
        name: String,
        path: String,
        packages: &mut Vec<Package>,
    ) -> Result<(), E>
    where
        E: de::Error,
    {
        let pkg = Package::new(name, Fetcher::CopyToStore { path });

        packages.push(pkg);

        Ok(())
    }

    /// # Deserialize a tarball package
    ///
    /// Deserialize a tarball package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_tarball_package<E>(
        _: String,
        url: String,
        packages: &mut Vec<Package>,
    ) -> Result<(), E>
    where
        E: de::Error,
    {
        let cmd_res = Command::new("nix")
            .args(["flake", "prefetch", &url, "--json"])
            .output()
            .map_err(|_| {
                de::Error::custom(
                    "Failed to fetch tarball contents for tarball package while deserializing",
                )
            })?;

        let stdout = str::from_utf8(&cmd_res.stdout).map_err(|_| {
            de::Error::custom("stdout was not valid utf8 while reading tarball hash")
        })?;

        let prefetch: Prefetch = serde_json::from_str(stdout).map_err(|err| {
            de::Error::custom(format!(
                "Failed to deserialize command result after calculating tarball hash {}",
                err
            ))
        })?;

        let name = format!("tarball:{}", url);

        let fetcher = Fetcher::new_tarball_package(url, prefetch.hash);
        let pkg = Package::new(name, fetcher);

        packages.push(pkg);

        Ok(())
    }

    /// # Deserialize a workspace package
    ///
    /// Deserialize a workspace package from it's bun lockfile representation
    ///
    /// This is found in the source as a tuple of arity 2
    pub fn deserialize_workspace_package<E>(self) -> Result<(), E>
    where
        E: de::Error,
    {
        let Some(id) = self.values[0].as_str() else {
            return Ok(());
        };

        let Some(path) = Self::drain_after_substring(id.to_string(), "workspace:") else {
            return Ok(());
        };

        let pkg = Package::new(self.name, Fetcher::CopyToStore { path });

        self.packages.push(pkg);

        Ok(())
    }

    fn drain_after_substring(mut input: String, sub: &str) -> Option<String> {
        let sub_start = input.find(sub)?;

        let sub_end = sub_start + sub.len();

        Some(input.drain(sub_end..).collect())
    }
}
