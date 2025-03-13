//! This module handles construction of the rendered nix code as the output

mod nix_escaper;
mod normalized_binary;

pub use nix_escaper::NixEscaper;
use normalized_binary::NormalizedBinary;
use rinja::Template;

use crate::{
    package::{Binaries, Fetched},
    Package,
};

/// # Nix Expression
///
/// A chunk of nix code to be written to stdout or a file
#[derive(Template)]
#[template(path = "output.nix")]
pub struct NixExpression {
    packages: Vec<Package<Fetched>>,
    binaries: Vec<NormalizedBinary>,
}

impl NixExpression {
    /// # New Nix Expression
    ///
    /// Produce a new, ready to render, nix expression from a fetch package list
    pub fn new(packages: Vec<Package<Fetched>>) -> Self {
        let normalized = packages
            .iter()
            .map(|pkg| (&pkg.name, &pkg.binaries))
            .collect::<Vec<_>>();

        let binaries = Self::normalize_binaries(normalized);

        Self { packages, binaries }
    }

    fn normalize_binaries<'a>(binaries: Vec<(&'a String, &'a Binaries)>) -> Vec<NormalizedBinary> {
        binaries
            .into_iter()
            .flat_map(|(name, bin)| match bin {
                Binaries::None => Vec::default(),
                Binaries::Unnamed(location) => vec![NormalizedBinary {
                    name: name.clone(),
                    location: location.clone(),
                }],
                Binaries::Named(map) => map
                    .iter()
                    .map(|(name, location)| NormalizedBinary {
                        name: name.clone(),
                        location: location.clone(),
                    })
                    .collect(),
            })
            .collect()
    }
}
