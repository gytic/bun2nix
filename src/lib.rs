#![warn(missing_docs)]

mod error;
mod lockfile;

pub use error::Result;
pub use lockfile::Lockfile;

/// # Convert Bun Lockfile to a Nix expression
///
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use Nix expression which fetches the packages
pub fn convert_lockfile_to_nix_expression(lockfile: String) -> Result<String> {
    let parsed: Lockfile = lockfile.parse()?;

    Ok(String::new())
}

