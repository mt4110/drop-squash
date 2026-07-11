use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourcePolicy {
    Keep,
    Trash,
    Ask,
}

impl SourcePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Trash => "trash",
            Self::Ask => "ask",
        }
    }
}

impl FromStr for SourcePolicy {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "keep" => Ok(Self::Keep),
            "trash" => Ok(Self::Trash),
            "ask" => Ok(Self::Ask),
            other => Err(format!("unknown source policy: {other}")),
        }
    }
}
