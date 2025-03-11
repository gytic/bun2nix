//! Library for implementing parsing and conversion of [Bun](https://bun.sh/) lock files into a
//! [Nix](https://en.wikipedia.org/wiki/Nix_(package_manager)) expression.

#![warn(missing_docs)]

mod error;
mod lockfile;
mod package;
mod prefetch;

use std::path::PathBuf;

pub use error::Result;
pub use lockfile::Lockfile;
pub use package::Package;
pub use prefetch::{DumpNixExpression, PrefetchedPackage};

use error::Error;

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

    let mut pkgs = lockfile.prefetch_packages(cache_location).await?;

    pkgs.sort_by(|a, b| a.hash.cmp(&b.hash));

    Ok(pkgs.dump_nix_expression())
}
