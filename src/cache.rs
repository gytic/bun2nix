//! Implementation of a simple [sqlite](https://www.sqlite.org/index.html) based cache for npm
//! packages that have already been fetched through nix

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use sqlx::{query_as, Connection, QueryBuilder, Sqlite, SqliteConnection};
use temp_packages_list::TempPackagesList;
use tokio::try_join;

use crate::{
    error::Result,
    package::{Fetched, Unfetched},
    FetchMany, Package,
};

mod cache_row;
mod cache_table;
mod temp_packages_list;

pub use cache_row::CacheRow;

/// # Cache
///
/// Struct representing the cache for this program
///
/// Carries out operations such as fetching packages that already exist in the cache, or creating
/// it if it does not exist
pub struct Cache {
    /// The location of the cache
    location: PathBuf,

    /// Connection to the cache held as long as the cache object exists
    connection: SqliteConnection,
}

impl Cache {
    /// # Create a new cache object
    ///
    /// This will create a new instance of the cache if it does not exist and migrate it with the
    /// exprected database schema
    pub async fn new(location: PathBuf) -> Result<Self> {
        let mut connection = Self::make_new_connection(&location).await?;

        sqlx::migrate!().run(&mut connection).await?;

        Ok(Self {
            connection,
            location,
        })
    }

    /// Create a new connection to the sqlite database
    async fn make_new_connection(location: &Path) -> Result<SqliteConnection> {
        Ok(SqliteConnection::connect(location.to_str().unwrap_or_default()).await?)
    }

    /// # List cached packages by npm identifier
    ///
    /// Returns an list of **every** single cache row currently in the database similar to a
    /// given npm identifier
    pub async fn list_cached_pkgs_by_npm_identifier(
        &mut self,
        npm_identifier: &str,
    ) -> Result<Vec<CacheRow>> {
        let query = format!("%{}%", npm_identifier);

        Ok(query_as!(
            CacheRow,
            "SELECT * FROM packages WHERE npm_identifier LIKE ?",
            query
        )
        .fetch_all(&mut self.connection)
        .await?)
    }

    /// # Delete all cached packages
    ///
    /// Completely clears the cache of all entries
    pub async fn delete_all_cached_packages(&mut self) -> Result<()> {
        query_as!(CacheRow, "DELETE FROM packages")
            .execute(&mut self.connection)
            .await?;

        Ok(())
    }

    /// # Fetch Packages
    ///
    /// Fetches packages from the cache if they exist, or fetches them through a nix prefetch if
    /// they don't and writes them to the cache
    pub async fn fetch_packages(
        mut self,
        packages: HashSet<Package<Unfetched>>,
    ) -> Result<Vec<Package<Fetched>>> {
        let npm_identifiers = packages
            .par_iter()
            .map(|pkg| pkg.npm_identifier.as_str())
            .collect::<HashSet<&str>>();

        let pkg_names_table = TempPackagesList::new(npm_identifiers, &mut self.connection).await?;

        let uncached_idents = pkg_names_table.list_uncached_npm_identifiers();

        let (uncached_pkgs, cached_pkgs): (Vec<_>, Vec<_>) = packages
            .into_par_iter()
            .partition(|pkg| uncached_idents.contains(&pkg.npm_identifier));

        let (cached, uncached) = try_join!(
            tokio::spawn(Self::fetch_cached_packages(
                self.connection,
                pkg_names_table,
                cached_pkgs
            )),
            tokio::spawn(Self::fetch_uncached_packages(self.location, uncached_pkgs))
        )?;

        let mut all = cached?;
        let mut uncached = uncached?;

        all.append(&mut uncached);

        Ok(all)
    }

    /// # Fetch Cached Packages
    ///
    /// Retrieve packages that have already been cached from the database and map them to the
    /// expected output names
    async fn fetch_cached_packages(
        mut connection: SqliteConnection,
        pkg_names_table: TempPackagesList,
        cached_pkgs: Vec<Package<Unfetched>>,
    ) -> Result<Vec<Package<Fetched>>> {
        let rows = pkg_names_table.list_cache_matches(&mut connection).await?;

        cached_pkgs
            .into_par_iter()
            .map(|pkg| {
                Package::<Fetched>::try_from_name_and_cache_row(
                    pkg.name,
                    rows.get(pkg.npm_identifier)?,
                )
            })
            .collect::<Result<Vec<Package<Fetched>>>>()
    }

    /// # Fetch Unached Packages
    ///
    /// Fetch packages that have yet to have been cached and then upload them afterwards
    async fn fetch_uncached_packages(
        location: PathBuf,
        uncached_pkgs: Vec<Package<Unfetched>>,
    ) -> Result<Vec<Package<Fetched>>> {
        let uncached = uncached_pkgs.fetch_many().await?;

        Self::upload_new_packages(location, &uncached).await?;

        Ok(uncached)
    }

    /// # Upload New Packages
    ///
    /// Upload fresh packages that haven't been seen yet to the cache
    async fn upload_new_packages(
        location: PathBuf,
        packages: &Vec<Package<Fetched>>,
    ) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        // Make a new connection as this is ran potentially at the same time as fetching in a
        // different thread so a unique connection is needed
        let mut connection = Self::make_new_connection(&location).await?;

        QueryBuilder::<Sqlite>::new(
            "INSERT OR IGNORE INTO packages (npm_identifier, url, hash, binaries) ",
        )
        .push_values(packages, |mut b, pkg| {
            b.push_bind(&pkg.npm_identifier);
            b.push_bind(&pkg.data.url);
            b.push_bind(&pkg.data.hash);
            b.push_bind(serde_json::to_string(&pkg.binaries).unwrap());
        })
        .build()
        .execute(&mut connection)
        .await?;

        Ok(())
    }
}
