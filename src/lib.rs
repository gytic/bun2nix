//! Library for implementing parsing and conversion of [Bun](https://bun.sh/) lock files into a
//! [Nix](https://en.wikipedia.org/wiki/Nix_(package_manager) expression

#![warn(missing_docs)]

mod error;
mod lockfile;

pub use error::Result;
pub use lockfile::Lockfile;

/// # Convert Bun Lockfile to a Nix expression
///
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
pub fn convert_lockfile_to_nix_expression(lockfile: String) -> Result<String> {
    let _parsed: Lockfile = lockfile.parse()?;

    Ok(String::new())
}

