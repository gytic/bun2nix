use std::collections::HashMap;

use crate::{Error, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sqlx::{query_as, SqliteConnection};

use super::CacheRow;

pub struct CacheTable(HashMap<String, (String, String, String)>);

impl CacheTable {
    pub async fn new(conn: &mut SqliteConnection) -> Result<Self> {
        let matches: Vec<(String, String, String, String)> = query_as(
            "SELECT p.npm_identifier, p.url, p.hash, p.binaries
            FROM packages p
            INNER JOIN temp_packages t ON p.npm_identifier = t.npm_identifier",
        )
        .fetch_all(conn)
        .await?;

        Ok(Self(
            matches
                .into_par_iter()
                .map(|row| (row.0, (row.1, row.2, row.3)))
                .collect(),
        ))
    }

    pub fn get(&self, npm_identifier: String) -> Result<CacheRow> {
        let row = self
            .0
            .get(&npm_identifier)
            .ok_or_else(|| Error::CacheTable(npm_identifier.clone()))?
            .clone();

        CacheRow::from_db_return(npm_identifier, row)
    }
}
