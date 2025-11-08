use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use std::process::Command;

/// # Package Prefetch
///
/// Represents the result of a `nix flake prefetch`
/// for a given package we don't know the hash for
#[derive(Debug, Deserialize, Serialize)]
pub struct Prefetch {
    pub hash: String,
}

impl Prefetch {
    /// # Prefetch Package
    ///
    /// Prefetch a package as a url and calculate it's
    /// sha256
    pub fn prefetch_package(url: &str) -> Result<Self> {
        let cmd_res = Command::new("nix")
            .args(["flake", "prefetch", url, "--json"])
            .output()
            .map_err(Error::FetchingFailed)?;

        let stdout = str::from_utf8(&cmd_res.stdout).map_err(Error::InvalidUtf8String)?;

        Ok(serde_json::from_str(stdout)?)
    }
}
