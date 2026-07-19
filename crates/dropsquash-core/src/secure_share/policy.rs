use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionPolicy {
    Safe,
    Unknown,
    Sensitive,
}

impl RegionPolicy {
    pub fn merged(self, other: Self) -> Self {
        self.max(other)
    }
}
