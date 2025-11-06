//! Errors which may occur during the running of this program
//!
//! This module contains two items:
//! - A giant unified error type `Error`
//! - An alias for `std::result::Result<T, E>` with that error for convenience

use thiserror::Error;

/// Result alias for Errors which occur in `bun2nix`
pub type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Error, Debug)]
/// Errors which occur in `bun2nix`
pub enum Error {
    #[error(
        "Failed to parse lockfile as JSONC (specified here: https://github.com/oven-sh/bun/issues/11863): {0}. Please make sure your bun lockfile is formatted correctly, try deleting it and running `bun install` again to produce a fresh one"
    )]
    ParseJsonc(#[from] jsonc_parser::errors::ParseError),
    #[error("Failed to parse lockfile related JSON as rust type: {0}")]
    ParseRustType(#[from] serde_json::Error),
    #[error(
        "Failed to parse empty lockfile, make sure you are providing a file with text contents"
    )]
    NoJsoncValue,
    #[error(
        "Missing @ for package name and version declaration. Make sure all versions in your bun lockfile are formatted properly or try deleting it and running `bun install` to produce a fresh one"
    )]
    NoAtInPackageIdentifier,
    #[error(
        "Unsupported lockfile version: '{0}'. Consider updating your local package or contributing to `bun2nix` if this version hasn't been supported yet"
    )]
    UnsupportedLockfileVersion(u8),
    #[error("Error while fetching package from it's source: {0}")]
    FetchingFailed(String),
    #[error("An invalid utf8 string was returned from stdin while fetching a package: {0}")]
    InvalidUtf8String(String),
    #[error("A workspace package was missing the `workspace:` specifier")]
    MissingWorkspaceSpecifier,
    #[error("Unexpected package entry length: {0}")]
    UnexpectedPackageEntryLength(usize),
    #[error("Failed to render template: '{0}'")]
    TemplateError(#[from] askama::Error),
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        JsValue::from_str(&err.to_string())
    }
}
