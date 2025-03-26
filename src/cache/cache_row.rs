use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{error::Error, package::Fetched, Package, Result};

#[derive(FromRow, Serialize, Deserialize, Debug)]
/// # Cache Row
///
/// An individual row in the cache for a package
pub struct CacheRow {
    /// The package's npm identifier
    pub npm_identifier: String,

    /// The url the package was fetched from
    pub url: String,

    /// The hash of the downloaded files
    pub hash: String,

    /// JSON representation of the binaries that should be created for this package
    pub binaries: String,
}

impl CacheRow {
    /// # From DB return
    ///
    /// Produce a new `CacheRow` instance from the structure contained in a database query
    /// return
    pub fn from_db_return(npm_identifier: String, body: (String, String, String)) -> Result<Self> {
        Ok(Self {
            npm_identifier,
            url: body.0,
            hash: body.1,
            binaries: body.2,
        })
    }
}

impl TryFrom<Package<Fetched>> for CacheRow {
    type Error = Error;

    fn try_from(pkg: Package<Fetched>) -> Result<Self> {
        Ok(Self {
            npm_identifier: pkg.npm_identifier,
            url: pkg.data.url,
            hash: pkg.data.hash,
            binaries: serde_json::to_string(&pkg.data.binaries)?,
        })
    }
}
