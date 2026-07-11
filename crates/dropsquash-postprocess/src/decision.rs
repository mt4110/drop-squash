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
            reason: "source movement safety gates were not satisfied".to_string(),
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
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn safe() -> SourceSafety {
        SourceSafety {
            conversion_succeeded: true,
            output_exists: true,
            original_bytes: 100,
            output_bytes: 40,
        }
    }

    fn action(policy: SourcePolicy, safety: SourceSafety) -> SourceAction {
        decide_source_action(PathBuf::from("input.mov"), policy, safety).action
    }

    #[test]
    fn keep_policy_never_moves_original() {
        assert_eq!(
            action(SourcePolicy::Keep, safe()),
            SourceAction::KeepOriginal
        );
    }

    #[test]
    fn ask_policy_requires_confirmation_when_safe() {
        assert_eq!(action(SourcePolicy::Ask, safe()), SourceAction::AskUser);
    }

    #[test]
    fn trash_policy_moves_only_when_safe() {
        assert_eq!(
            action(SourcePolicy::Trash, safe()),
            SourceAction::MoveOriginalToTrash
        );
    }

    #[test]
    fn failed_conversion_keeps_original() {
        let mut safety = safe();
        safety.conversion_succeeded = false;
        assert_eq!(
            action(SourcePolicy::Trash, safety),
            SourceAction::KeepOriginal
        );
    }

    #[test]
    fn missing_output_keeps_original() {
        let mut safety = safe();
        safety.output_exists = false;
        assert_eq!(
            action(SourcePolicy::Trash, safety),
            SourceAction::KeepOriginal
        );
    }

    #[test]
    fn zero_output_keeps_original() {
        let mut safety = safe();
        safety.output_bytes = 0;
        assert_eq!(
            action(SourcePolicy::Trash, safety),
            SourceAction::KeepOriginal
        );
    }

    #[test]
    fn zero_original_keeps_original() {
        let mut safety = safe();
        safety.original_bytes = 0;
        assert_eq!(
            action(SourcePolicy::Trash, safety),
            SourceAction::KeepOriginal
        );
    }

    #[test]
    fn larger_output_keeps_original() {
        let mut safety = safe();
        safety.output_bytes = 120;
        assert_eq!(
            action(SourcePolicy::Trash, safety),
            SourceAction::KeepOriginal
        );
    }
}
