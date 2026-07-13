use std::path::PathBuf;

use dropsquash_core::SourcePolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSafety {
    pub conversion_succeeded: bool,
    pub output_exists: bool,
    pub original_bytes: u64,
    pub output_bytes: u64,
}

impl SourceSafety {
    pub fn permits_original_movement(&self) -> bool {
        self.conversion_succeeded
            && self.output_exists
            && self.output_bytes > 0
            && self.original_bytes > 0
            && self.output_bytes < self.original_bytes
    }

    fn failure_reason(&self) -> &'static str {
        if !self.conversion_succeeded {
            return "conversion did not succeed";
        }
        if !self.output_exists {
            return "output file is missing";
        }
        if self.original_bytes == 0 {
            return "original size is unavailable";
        }
        if self.output_bytes == 0 {
            return "output size is zero";
        }
        if self.output_bytes >= self.original_bytes {
            return "output is not smaller than original";
        }
        "source movement safety gates were not satisfied"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceAction {
    KeepOriginal,
    AskUser,
    MoveOriginalToTrash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceActionDecision {
    pub action: SourceAction,
    pub reason: String,
    pub source_path: PathBuf,
}

pub fn decide_source_action(
    source_path: PathBuf,
    policy: SourcePolicy,
    safety: SourceSafety,
) -> SourceActionDecision {
    if !safety.permits_original_movement() {
        return SourceActionDecision {
            action: SourceAction::KeepOriginal,
            reason: safety.failure_reason().to_string(),
            source_path,
        };
    }

    match policy {
        SourcePolicy::Keep => SourceActionDecision {
            action: SourceAction::KeepOriginal,
            reason: "source policy is keep".to_string(),
            source_path,
        },
        SourcePolicy::Ask => SourceActionDecision {
            action: SourceAction::AskUser,
            reason: "source policy requires user confirmation".to_string(),
            source_path,
        },
        SourcePolicy::Trash => SourceActionDecision {
            action: SourceAction::MoveOriginalToTrash,
            reason: "source policy permits trash after successful conversion".to_string(),
            source_path,
        },
    }
}

#[cfg(test)]
mod tests;
