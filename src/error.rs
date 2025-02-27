use thiserror::Error;

/// Result alias for Errors which occur in `bun2nix`
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Errors which occur in `bun2nix`
pub enum Error {
    #[error("Failed to parse lockfile as JSONC (specified here: https://github.com/oven-sh/bun/issues/11863)")]
    ParseJsonc(#[from] jsonc_parser::errors::ParseError),
    #[error("Failed to parse lockfile JSONC as rust type ")]
    ParseRustType(#[from] serde_json::Error),
    #[error("Failed to parse empty lockfile")]
    NoJsoncValue,
}
