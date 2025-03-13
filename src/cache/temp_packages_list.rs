use std::collections::HashSet;

use sqlx::{query_as, Executor, QueryBuilder, Sqlite, SqliteConnection};

use crate::Result;

use super::cache_table::CacheTable;

pub struct TempPackagesList(Vec<String>);

impl TempPackagesList {
    pub async fn new(
        npm_identifiers: HashSet<&str>,
        connection: &mut SqliteConnection,
    ) -> Result<Self> {
        connection
            .execute("CREATE TEMP TABLE temp_packages (npm_identifier TEXT NOT NULL PRIMARY KEY)")
            .await?;

        QueryBuilder::<Sqlite>::new("INSERT OR IGNORE INTO temp_packages (npm_identifier) ")
            .push_values(&npm_identifiers, |mut b, npm_identifier| {
                b.push_bind(npm_identifier);
            })
            .build()
            .execute(&mut *connection)
            .await?;

        Ok(Self(
            Self::fetch_uncached_npm_identifiers(connection).await?,
        ))
    }

    async fn fetch_uncached_npm_identifiers(
        connection: &mut SqliteConnection,
    ) -> Result<Vec<String>> {
        Ok(query_as::<_, (String,)>(
            "SELECT DISTINCT t.npm_identifier
             FROM temp_packages t
             LEFT JOIN packages p ON t.npm_identifier = p.npm_identifier
             WHERE p.npm_identifier IS NULL",
        )
        .fetch_all(connection)
        .await?
        .into_iter()
        .map(|row| row.0)
        .collect())
    }

    pub async fn list_cache_matches(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<CacheTable> {
        CacheTable::new(connection).await
    }

    pub fn list_uncached_npm_identifiers(&self) -> &Vec<String> {
        &self.0
    }
}
