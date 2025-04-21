use std::fmt;

use serde::de::{self, MapAccess, Visitor};

use crate::{
    package::{Extracted, MetaData},
    Package,
};

/// # Package Visitor
///
/// Used for a custom serde deserialize method as the most ergonomic rust package data type does
/// not match the type in the lockfile directly
pub struct PackageVisitor;

impl<'de> Visitor<'de> for PackageVisitor {
    type Value = Vec<Package<Extracted>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of package names to tuples")
    }

    fn visit_map<M>(self, mut map: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut packages = Vec::new();

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

            let meta: MetaData = serde_json::from_str(&values[2].to_string())
                .map_err(|e| de::Error::custom(format!("Invalid metadata format: {}", e)))?;

            let hash = values[3]
                .as_str()
                .ok_or_else(|| de::Error::custom("Invalid hash format"))?
                .to_string();

            assert!(
                hash.contains("sha512-"),
                "Expected hash to be in sri format and contain sha512"
            );

            let pkg = Package::new(name, npm_identifier, hash, meta.binaries);

            packages.push(pkg);
        }

        Ok(packages)
    }
}
