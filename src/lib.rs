//! Library for implementing parsing and conversion of [Bun](https://bun.sh/) lock files into a
//! [Nix](https://en.wikipedia.org/wiki/Nix_(package_manager)) expression.

#![warn(missing_docs)]

mod cache;
pub mod error;
mod lockfile;
mod package;

use std::path::PathBuf;

pub use cache::Cache;
pub use error::Result;
pub use lockfile::Lockfile;
pub use package::Package;

/// # Convert Bun Lockfile to a Nix expression
///
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
pub async fn convert_lockfile_to_nix_expression(
    contents: String,
    cache_location: Option<PathBuf>,
) -> Result<String> {
    let lockfile = contents.parse::<Lockfile>()?;

    if lockfile.lockfile_version != 1 {
        return Err(Error::UnsupportedLockfileVersion(lockfile.lockfile_version));
    };

    let mut fods = Cache::prefetch_packages(lockfile.packages(), cache_location).await?;

    fods.sort_by(|a, b| a.data.hash.cmp(&b.data.hash));

    Ok(fods.dump_nix_expression())
}
