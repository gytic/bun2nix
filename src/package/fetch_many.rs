use std::collections::HashSet;

use futures::{stream, StreamExt, TryStreamExt};

use crate::{
    package::{Fetched, Unfetched},
    Package, Result,
};

const CONCURRENT_FETCH_REQUESTS: usize = 100;

/// # Fetch Many
///
/// This trait provides an abstraction for collections of `Package`s to implement that fetchs many
/// packages concurrently
pub trait FetchMany {
    /// Fetch many instances of a package and turn them into their fetched variant
    ///
    /// This should be consumed with `async fn ...`
    fn fetch_many(self) -> impl std::future::Future<Output = Result<Vec<Package<Fetched>>>>;
}

impl FetchMany for Vec<Package<Unfetched>> {
    async fn fetch_many(self) -> Result<Vec<Package<Fetched>>> {
        stream::iter(self)
            .map(Package::<Unfetched>::fetch_one)
            .buffer_unordered(CONCURRENT_FETCH_REQUESTS)
            .try_collect()
            .await
    }
}

impl FetchMany for HashSet<Package<Unfetched>> {
    async fn fetch_many(self) -> Result<Vec<Package<Fetched>>> {
        stream::iter(self)
            .map(Package::<Unfetched>::fetch_one)
            .buffer_unordered(CONCURRENT_FETCH_REQUESTS)
            .try_collect()
            .await
    }
}
