use serde::{Deserialize, Serialize};

use super::State;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
/// Data held by an unfetched package
pub struct Unfetched;

impl State for Unfetched {}
