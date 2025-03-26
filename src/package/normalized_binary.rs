use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// # Normalized Binary
///
/// Normal version of a binary symlink with a name pointing to a location
pub struct NormalizedBinary {
    /// The file name to create the symlink under
    pub name: String,

    /// The actual file to point the symlink to
    pub location: String,
}

impl Hash for NormalizedBinary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
