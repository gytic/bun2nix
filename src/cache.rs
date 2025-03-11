use std::{collections::HashSet, path::PathBuf};

use futures::{stream, StreamExt, TryStreamExt};
use sqlx::{query_as, Connection, Executor, QueryBuilder, Sqlite, SqliteConnection};

const CONCURRENT_FETCH_REQUESTS: usize = 100;

use crate::{
    error::Result,
    package::{Fetched, Unfetched},
    Package,
};

pub struct Cache;

impl Cache {
    /// Use the lockfile's packages to produce prefetched sha256s for each
    pub async fn prefetch_packages(
        mut packages: HashSet<Package<Unfetched>>,
        cache_location: Option<PathBuf>,
    ) -> Result<Vec<Package<Fetched>>> {
        let Some(loc) = cache_location else {
            return Self::fetch_uncached_packages(packages, None).await;
        };

        let mut cache = Self::connect_and_migrate(loc).await?;
        Self::create_temp_pkg_list_db(&packages, &mut cache).await?;

        let mut cached: Vec<Package<Fetched>> = query_as(
            "SELECT p.npm_identifier, p.url, p.hash, p.binaries
            FROM packages p
            INNER JOIN temp_packages t ON p.npm_identifier = t.npm_identifier",
        )
        .fetch_all(&mut cache)
        .await?;

        let uncached_names = query_as::<_, (String,)>(
            "SELECT DISTINCT t.npm_identifier
             FROM temp_packages t
             LEFT JOIN packages p ON t.npm_identifier = p.npm_identifier
             WHERE p.npm_identifier IS NULL",
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
        packages: &HashSet<Package<Unfetched>>,
        cache: &mut SqliteConnection,
    ) -> Result<()> {
        let npm_idents = packages
            .iter()
            .map(|pkg| &pkg.npm_identifier)
            .collect::<HashSet<_>>();

        cache
            .execute("CREATE TEMP TABLE temp_packages (npm_identifier TEXT NOT NULL PRIMARY KEY)")
            .await?;

        QueryBuilder::<Sqlite>::new("INSERT INTO temp_packages (npm_identifier) ")
            .push_values(npm_idents, |mut b, npm_identifier| {
                b.push_bind(npm_identifier);
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
        packages: HashSet<Package<Unfetched>>,
        cache: Option<SqliteConnection>,
    ) -> Result<Vec<PrefetchedPackage>> {
        let pkgs = stream::iter(packages)
            .map(PrefetchedPackage::nix_store_fetch)
            .buffer_unordered(CONCURRENT_FETCH_REQUESTS)
            .try_collect()
            .await?;

        let Some(mut cache) = cache else {
            return Ok(pkgs);
        };

        let to_insert = pkgs.iter().collect::<HashSet<_>>();

        QueryBuilder::<Sqlite>::new("INSERT INTO packages (npm_identifier, url, hash, binaries) ")
            .push_values(to_insert, |mut b, pkg| {
                b.push_bind(&pkg.npm_identifier);
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
