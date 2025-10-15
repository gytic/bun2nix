//! This module holds the implementation for data about a given nix fetcher type

mod copy_to_store;
mod fetch_url;

pub use copy_to_store::CopyToStore;
pub use fetch_url::FetchUrl;

use std::{fmt::Debug, hash::Hash};

use rinja::Template;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
/// # Package Fetcher
///
/// Nix-translated fetcher for a given package
pub enum Fetcher {
    FetchUrl(FetchUrl),
    CopyToStore(CopyToStore),
    #[default]
    Unknown,
}

impl Fetcher {
    pub fn render(&self) -> rinja::Result<String> {
        match self {
            Self::FetchUrl(fetcher) => fetcher.render(),
            Self::CopyToStore(fetcher) => fetcher.render(),
            Self::Unknown => Ok(String::default()),
        }
    }
}
