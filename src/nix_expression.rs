//! This module handles construction of the rendered nix code as the output

mod nix_escaper;

pub use crate::error::Result;
pub use nix_escaper::NixEscaper;
use rinja::Template;

use crate::Package;

/// # Nix Expression
///
/// A chunk of nix code to be written to stdout or a file
#[derive(Template)]
#[template(path = "output.jinja")]
pub struct NixExpression {
    packages: Vec<Package>,
}

impl NixExpression {
    /// # New Nix Expression
    ///
    /// Produce a new, ready to render, nix expression from a package list
    pub fn new(packages: Vec<Package>) -> Result<Self> {
        Ok(Self { packages })
    }
}
