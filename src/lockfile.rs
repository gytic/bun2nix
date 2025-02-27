use std::{collections::HashMap, process::Command, str::FromStr};

use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::{Error, Result},
    PrefetchOutput,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// # Bun Lockfile
///
/// A model of the fields that exist in a bun lockfile in order to serve as a deserialization
/// target
pub struct Lockfile {
    lockfile_version: u8,
    #[serde(default)]
    workspaces: HashMap<String, Workspace>,
    #[serde(default)]
    packages: HashMap<String, Package>,
}

impl Lockfile {
    fn parse_to_value(lockfile: &str) -> Result<Value> {
        jsonc_parser::parse_to_serde_value(lockfile, &Default::default())?
            .ok_or(Error::NoJsoncValue)
    }

    /// Use the lockfile's packages to produce prefetched sha256s for each
    pub fn prefetch_packages(&self) -> Result<Vec<PrefetchOutput>> {
        self.packages
            .values()
            .par_bridge()
            .map(|p| -> Result<PrefetchOutput> {
                let url = p.to_npm_url()?;

                let output = Command::new("nix")
                    .args(["flake", "prefetch", &url, "--json"])
                    .output()?;

                let stdout = String::from_utf8(output.stdout)?;

                Ok(serde_json::from_str(&stdout)?)
            })
            .collect()
    }
}

impl FromStr for Lockfile {
    type Err = Error;

    fn from_str(lockfile: &str) -> std::result::Result<Self, Self::Err> {
        let value = Self::parse_to_value(lockfile)?;

        Ok(serde_json::from_value(value)?)
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Workspace {
    name: Option<String>,
    dependencies: HashMap<String, String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Package(String, String, Peers, String);

impl Package {
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
        let Some((user, name_and_ver)) = self.0.split_once("/") else {
            let Some((name, ver)) = self.0.split_once("@") else {
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

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Peers {
    peer_dependencies: HashMap<String, String>,
    optional_peers: Vec<String>,
}

#[test]
fn test_parse_to_value_with_sample() {
    let sample = r#"
        // Allow comments as per jsonc spec
        {
            "name": "John Doe",
            "age": 43,
        }"#;

    let value = Lockfile::parse_to_value(sample).unwrap();

    assert!(value["name"] == "John Doe");
    assert!(value["age"] == 43);
}

#[test]
fn test_parse_to_value_empty() {
    let sample = "";

    let value = Lockfile::parse_to_value(sample).unwrap_err();

    assert!(value.to_string() == "Failed to parse empty lockfile");
}

#[test]
fn test_from_str_version_only() {
    let lockfile = r#"
        {
            "lockfileVersion": 1,
        }"#;

    let value: Lockfile = lockfile.parse().unwrap();

    assert!(value.lockfile_version == 1);
}

#[test]
fn test_prefetch_packages() {
    let lockfile = r#"
        {
          "lockfileVersion": 1,
          "workspaces": {
            "": {
              "name": "examples",
              "devDependencies": {
                "@types/bun": "latest",
              },
              "peerDependencies": {
                "typescript": "^5",
              },
            },
          },
          "packages": {
            "@types/bun": ["@types/bun@1.2.4", "", { "dependencies": { "bun-types": "1.2.4" } }, "sha512-QtuV5OMR8/rdKJs213iwXDpfVvnskPXY/S0ZiFbsTjQZycuqPbMW8Gf/XhLfwE5njW8sxI2WjISURXPlHypMFA=="],

            "@types/node": ["@types/node@22.13.5", "", { "dependencies": { "undici-types": "~6.20.0" } }, "sha512-+lTU0PxZXn0Dr1NBtC7Y8cR21AJr87dLLU953CWA6pMxxv/UDc7jYAY90upcrie1nRcD6XNG5HOYEDtgW5TxAg=="],

            "@types/ws": ["@types/ws@8.5.14", "", { "dependencies": { "@types/node": "*" } }, "sha512-bd/YFLW+URhBzMXurx7lWByOu+xzU9+kb3RboOteXYDfW+tr+JZa99OyNmPINEGB/ahzKrEuc8rcv4gnpJmxTw=="],

            "bun-types": ["bun-types@1.2.4", "", { "dependencies": { "@types/node": "*", "@types/ws": "~8.5.10" } }, "sha512-nDPymR207ZZEoWD4AavvEaa/KZe/qlrbMSchqpQwovPZCKc7pwMoENjEtHgMKaAjJhy+x6vfqSBA1QU3bJgs0Q=="],

            "typescript": ["typescript@5.7.3", "", { "bin": { "tsc": "bin/tsc", "tsserver": "bin/tsserver" } }, "sha512-84MVSjMEHP+FQRPy3pX9sTVV/INIex71s9TL2Gm5FG/WG1SqXeKyZ0k7/blY/4FdOzI12CBy1vGc4og/eus0fw=="],

            "undici-types": ["undici-types@6.20.0", "", {}, "sha512-Ny6QZ2Nju20vw1SRHe3d9jVu6gJ+4e3+MMpqu7pqE5HT6WsTSlce++GQmK5UXS8mzV8DSYHrQH+Xrf2jVcuKNg=="],
          }
        }
    "#;

    let value: Lockfile = lockfile.parse().unwrap();

    assert!(value.lockfile_version == 1);
    assert!(value.workspaces[""].name == Some(String::from("examples")));
    assert!(value.packages["@types/bun"].0 == "@types/bun@1.2.4");

    let prefetched = value.prefetch_packages().unwrap();

    assert!(!prefetched.is_empty());
}

#[test]
fn test_to_npm_url() {
    let package = Package(
        "bun-types@1.2.4".to_owned(),
        "".to_owned(),
        Peers::default(),
        "".to_owned(),
    );

    let out = package.to_npm_url().unwrap();

    assert!(out == "https://registry.npmjs.org/bun-types/-/bun-types-1.2.4.tgz");
}

#[test]
fn test_to_npm_url_with_namespace() {
    let package = Package(
        "@alloc/quick-lru@5.2.0".to_owned(),
        "".to_owned(),
        Peers::default(),
        "".to_owned(),
    );

    let out = package.to_npm_url().unwrap();

    assert!(out == "https://registry.npmjs.org/@alloc/quick-lru/-/quick-lru-5.2.0.tgz");
}
