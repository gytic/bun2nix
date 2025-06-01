//! This module handles construction of the rendered nix code as the output

mod nix_escaper;

pub use crate::{error::Result, package::NormalizedBinary};
pub use nix_escaper::NixEscaper;
use rinja::Template;

use crate::{
    Package,
    package::{Extracted, Normalized},
};

/// # Nix Expression
///
/// A chunk of nix code to be written to stdout or a file
#[derive(Template)]
#[template(path = "output.nix_template")]
pub struct NixExpression {
    packages: Vec<Package<Normalized>>,
}

impl NixExpression {
    /// # New Nix Expression
    ///
    /// Produce a new, ready to render, nix expression from a package list
    pub fn new(packages: Vec<Package<Extracted>>) -> Result<Self> {
        let packages = packages
            .into_iter()
            .map(|pkg| pkg.normalize())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { packages })
    }
}
