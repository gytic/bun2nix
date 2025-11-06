use std::fmt;

use serde::de::{self, MapAccess, Visitor};

use super::PackageDeserializer;
use crate::Package;

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
        let mut packages: Vec<Package> = Vec::new();

        while let Some((name, values)) = map.next_entry::<String, Vec<serde_json::Value>>()? {
            let arity = values.len();

            let deserializer = PackageDeserializer {
                packages: &mut packages,
                name,
                values,
            };

            match arity {
                1 => deserializer.deserialize_workspace_package()?,
                2 => deserializer.deserialize_tarball_or_file_package()?,
                4 => deserializer.deserialize_npm_package()?,
                _ => {
                    return Err(de::Error::custom(
                        "Invalid package entry for expected at least 4 values",
                    ));
                }
            };
        }

        Ok(packages)
    }
}
