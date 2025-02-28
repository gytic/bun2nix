//! Library for implementing parsing and conversion of [Bun](https://bun.sh/) lock files into a
//! [Nix](https://en.wikipedia.org/wiki/Nix_(package_manager) expression

#![warn(missing_docs)]

mod error;
mod lockfile;
mod prefetch;

pub use error::Result;
pub use lockfile::Lockfile;
pub use prefetch::{DumpNixExpression, PrefetchOutput};

/// # Convert Bun Lockfile to a Nix expression
///
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
pub fn convert_lockfile_to_nix_expression(contents: String) -> Result<String> {
    let mut pkgs = contents.parse::<Lockfile>()?.prefetch_packages()?;

    pkgs.sort_by(|a, b| a.hash.cmp(&b.hash));

    Ok(pkgs.dump_nix_expression())
}
