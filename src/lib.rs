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
/// Takes a string input of the contents of a bun lockfile and converts it into a ready to use formatted Nix expression which fetches the packages
pub fn convert_lockfile_to_nix_expression(contents: String) -> Result<String> {
    let nix_expr = contents
        .parse::<Lockfile>()?
        .prefetch_packages()?
        .dump_nix_expression();

    Ok(nixpkgs_fmt::reformat_string(&nix_expr))
}
