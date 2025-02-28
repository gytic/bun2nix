use thiserror::Error;

/// Result alias for Errors which occur in `bun2nix`
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Errors which occur in `bun2nix`
pub enum Error {
    #[error("Failed to parse lockfile as JSONC (specified here: https://github.com/oven-sh/bun/issues/11863): {}", 0)]
    ParseJsonc(#[from] jsonc_parser::errors::ParseError),
    #[error("Failed to parse lockfile JSONC as rust type: {}", 0)]
    ParseRustType(#[from] serde_json::Error),
    #[error("Failed to parse empty lockfile")]
    NoJsoncValue,
    #[error("Missing @ for package name and version declaration")]
    NoAtInPackageIdentifier,
    #[error("Error occurred in prefetch command: {}", 0)]
    Prefetch(#[from] std::io::Error),
    #[error("Error parsing UTF8: {}", 0)]
    UTF8Parse(#[from] std::string::FromUtf8Error),
    #[error(
        "Unsupported lockfile version: '{}'. Consider updating your local package or contributing to `bun2nix` if this version hasn't been supported yet.",
        0
    )]
    UnsupportedLockfileVersion(u8),
}
