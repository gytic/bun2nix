use std::{fmt, process::Command};

use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, Visitor},
};

use crate::{Package, package::Fetcher};

/// # Package Visitor
///
/// Used for a custom serde deserialize method as the most ergonomic rust package data type does
/// not match the type in the lockfile directly
pub struct PackageVisitor;

impl<'de> Visitor<'de> for PackageVisitor {
    type Value = Vec<Package>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of package names to tuples")
    }

    fn visit_map<M>(self, mut map: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut packages = Vec::new();

        while let Some((name, values)) = map.next_entry::<String, Vec<serde_json::Value>>()? {
            match values.len() {
                1 => deserialize_workspace_package(name, values, &mut packages)?,
                2 => deserialize_tarball_or_file_package(name, values, &mut packages)?,
                4 => deserialize_npm_package(values, &mut packages)?,
                _ => {
                    return Err(de::Error::custom(format!(
                        "Invalid package entry for {}: expected at least 4 values",
                        name
                    )));
                }
            };
        }

        Ok(packages)
    }
}

fn deserialize_tarball_or_file_package<E>(
    name: String,
    values: Vec<serde_json::Value>,
    packages: &mut Vec<Package>,
) -> Result<(), E>
where
    E: de::Error,
{
    let Some(id) = values[0].as_str() else {
        return Ok(());
    };

    let Some(path) = drain_after_substring(id.to_string(), "@") else {
        return Ok(());
    };

    if path.starts_with("http") {
        deserialize_tarball_package(name, path, packages)
    } else {
        deserialize_file_package(name, path, packages)
    }
}

fn drain_after_substring(mut input: String, sub: &str) -> Option<String> {
    let sub_start = input.find(sub)?;

    let sub_end = sub_start + sub.len();

    Some(input.drain(sub_end..).collect())
}

fn deserialize_workspace_package<E>(
    name: String,
    values: Vec<serde_json::Value>,
    packages: &mut Vec<Package>,
) -> Result<(), E>
where
    E: de::Error,
{
    let Some(id) = values[0].as_str() else {
        return Ok(());
    };

    let Some(path) = drain_after_substring(id.to_string(), "workspace:") else {
        return Ok(());
    };

    let pkg = Package::new(name, Fetcher::CopyToStore { path });

    packages.push(pkg);

    Ok(())
}

fn deserialize_file_package<E>(
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

#[derive(Debug, Deserialize, Serialize)]
struct Prefetch {
    hash: String,
}

fn deserialize_tarball_package<E>(
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

    let stdout = str::from_utf8(&cmd_res.stdout)
        .map_err(|_| de::Error::custom("stdout was not valid utf8 while reading tarball hash"))?;

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

fn deserialize_npm_package<E>(
    values: Vec<serde_json::Value>,
    packages: &mut Vec<Package>,
) -> Result<(), E>
where
    E: de::Error,
{
    let npm_identifier_raw = values[0]
        .as_str()
        .ok_or_else(|| de::Error::custom("Invalid npm_identifier format"))?
        .to_string();

    let hash = values[3]
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
    packages.push(pkg);

    Ok(())
}
