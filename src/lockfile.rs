use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use futures::{stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sqlx::{query_as, Connection, Executor, QueryBuilder, Sqlite, SqliteConnection};

use crate::{
    error::{Error, Result},
    package::PackagesVisitor,
    Package, PrefetchedPackage,
};

const CONCURRENT_FETCH_REQUESTS: usize = 100;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// # Bun Lockfile
///
/// A model of the fields that exist in a bun lockfile in order to serve as a deserialization
/// target
pub struct Lockfile {
    /// The version field of the bun lockfile
    pub lockfile_version: u8,

    /// The workspaces declaration in the bun lockfile
    #[serde(default)]
    pub workspaces: HashMap<String, Workspace>,

    /// The list of all packages needed by the lockfile
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_packages")]
    pub packages: HashSet<Package>,
}

fn deserialize_packages<'de, D>(data: D) -> std::result::Result<HashSet<Package>, D::Error>
where
    D: Deserializer<'de>,
{
    data.deserialize_map(PackagesVisitor)
}

impl Lockfile {
    fn parse_to_value(lockfile: &str) -> Result<Value> {
        jsonc_parser::parse_to_serde_value(lockfile, &Default::default())?
            .ok_or(Error::NoJsoncValue)
    }

    /// Use the lockfile's packages to produce prefetched sha256s for each
    pub async fn prefetch_packages(
        self,
        cache_location: Option<PathBuf>,
    ) -> Result<Vec<PrefetchedPackage>> {
        println!("pkgs: {:#?}", self.packages);

        let mut packages = self.packages;

        let Some(loc) = cache_location else {
            return Self::fetch_uncached_packages(packages, None).await;
        };

        let mut cache = Self::connect_and_migrate(loc).await?;
        Self::create_temp_pkg_list_db(&packages, &mut cache).await?;

        let mut cached: Vec<PrefetchedPackage> = query_as(
            "SELECT p.name, p.url, p.hash, p.binaries
            FROM packages p
            INNER JOIN temp_packages t ON p.name = t.name",
        )
        .fetch_all(&mut cache)
        .await?;

        let uncached_names = query_as::<_, (String,)>(
            "SELECT DISTINCT t.name
             FROM temp_packages t
             LEFT JOIN packages p ON t.name = p.name
             WHERE p.name IS NULL",
        )
        .fetch_all(&mut cache)
        .await?
        .into_iter()
        .map(|x| x.0)
        .collect::<HashSet<_>>();

        packages.retain(|pkg| uncached_names.contains(&pkg.npm_identifier));

        if packages.is_empty() {
            return Ok(cached);
        };

        let new_pkgs = Self::fetch_uncached_packages(packages, Some(cache)).await?;

        cached.extend(new_pkgs);

        Ok(cached)
    }

    async fn create_temp_pkg_list_db(
        packages: &HashSet<Package>,
        cache: &mut SqliteConnection,
    ) -> Result<()> {
        cache
            .execute("CREATE TEMP TABLE temp_packages (name TEXT NOT NULL PRIMARY KEY)")
            .await?;

        QueryBuilder::<Sqlite>::new("INSERT INTO temp_packages (name) ")
            .push_values(packages, |mut b, package| {
                b.push_bind(&package.npm_identifier);
            })
            .build()
            .execute(cache)
            .await?;

        Ok(())
    }

    async fn connect_and_migrate(loc: PathBuf) -> Result<SqliteConnection> {
        let mut conn = SqliteConnection::connect(loc.to_str().unwrap_or_default()).await?;

        sqlx::migrate!().run(&mut conn).await?;

        Ok(conn)
    }

    async fn fetch_uncached_packages(
        packages: HashSet<Package>,
        cache: Option<SqliteConnection>,
    ) -> Result<Vec<PrefetchedPackage>> {
        let pkgs = stream::iter(packages)
            .map(|package| async {
                let url = package.to_npm_url()?;

                PrefetchedPackage::nix_store_fetch(package.npm_identifier, url, package.meta.bin)
                    .await
            })
            .buffer_unordered(CONCURRENT_FETCH_REQUESTS)
            .try_collect()
            .await?;

        let Some(mut cache) = cache else {
            return Ok(pkgs);
        };

        QueryBuilder::<Sqlite>::new("INSERT INTO packages (name, url, hash, binaries) ")
            .push_values(&pkgs, |mut b, pkg| {
                b.push_bind(&pkg.name);
                b.push_bind(&pkg.url);
                b.push_bind(&pkg.hash);
                b.push_bind(serde_json::to_string(&pkg.binaries).unwrap());
            })
            .build()
            .execute(&mut cache)
            .await?;

        Ok(pkgs)
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
    /// The name of the workspace
    pub name: Option<String>,
    dependencies: HashMap<String, String>,
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

    assert!(value.to_string() == "Failed to parse empty lockfile, make sure you are providing a file with text contents.");
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
