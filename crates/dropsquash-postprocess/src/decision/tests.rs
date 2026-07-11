use std::path::PathBuf;

use dropsquash_core::SourcePolicy;

use super::{decide_source_action, SourceAction, SourceSafety};

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

#[test]
fn equal_size_output_keeps_original() {
    let mut safety = safe();
    safety.output_bytes = safety.original_bytes;
    assert_eq!(
        action(SourcePolicy::Trash, safety),
        SourceAction::KeepOriginal
    );
}
