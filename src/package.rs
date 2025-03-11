use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::{Hash, Hasher},
};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize,
};

use crate::error::{Error, Result};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
/// # Lockfile Package
///
/// An individual package found in a bun lockfile
pub struct Package {
    /// The name of the package, as found in the `./node_modules` directory or in an import
    /// statement
    pub name: String,

    /// The package's identifier string for fetching from npm
    pub npm_identifier: String,

    /// Package metadata
    pub meta: MetaData,
}

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

pub struct PackagesVisitor;

impl<'de> Visitor<'de> for PackagesVisitor {
    type Value = HashSet<Package>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of package names to tuples")
    }

    fn visit_map<M>(self, mut map: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut packages = HashSet::new();

        while let Some((name, values)) = map.next_entry::<String, Vec<serde_json::Value>>()? {
            if values.len() < 4 {
                return Err(de::Error::custom(format!(
                    "Invalid package entry for {}: expected at least 4 values",
                    name
                )));
            }

            let npm_identifier = values[0]
                .as_str()
                .ok_or_else(|| de::Error::custom("Invalid npm_identifier format"))?
                .to_string();

            println!("meta: {:#?}", values[2]);

            let meta = serde_json::from_str(values[2].as_str().unwrap_or_default())
                .map_err(|_| de::Error::custom("Invalid metadata format"))?;

            packages.insert(Package {
                name,
                npm_identifier,
                meta,
            });
        }

        Ok(packages)
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Package {}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MetaData {
    pub peer_dependencies: HashMap<String, String>,
    pub optional_peers: Vec<String>,
    pub bin: Binaries,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Binaries {
    #[default]
    None,
    Unnamed(String),
    Named(HashMap<String, String>),
}

impl TryFrom<String> for Binaries {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(serde_json::from_str(&value)?)
    }
}
