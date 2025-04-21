//! Library for implementing parsing and conversion of [Bun](https://bun.sh/) lock files into a
//! [Nix](https://en.wikipedia.org/wiki/Nix_(package_manager)) expression.

#![warn(missing_docs)]

pub mod error;
pub mod lockfile;
pub mod nix_expression;
pub mod package;

pub use error::{Error, Result};
pub use lockfile::Lockfile;
use nix_expression::NixExpression;
pub use package::Package;
use rinja::Template;

/// # Convert Bun Lockfile to a Nix expression
///
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
pub fn convert_lockfile_to_nix_expression(contents: String) -> Result<String> {
    let lockfile = contents.parse::<Lockfile>()?;

    if lockfile.lockfile_version != 1 {
        return Err(Error::UnsupportedLockfileVersion(lockfile.lockfile_version));
    };

    let mut packages = lockfile.packages();
    packages.sort();

    Ok(NixExpression::new(packages)?.render()?)
}
