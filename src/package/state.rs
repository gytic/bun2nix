mod fetched;
mod normalized;
mod unfetched;

pub use fetched::Fetched;
pub use normalized::Normalized;
pub use unfetched::Unfetched;

pub trait State {}
