use std::path::PathBuf;

use dropsquash_core::SourcePolicy;

use super::{decide_source_action, SourceAction, SourceActionDecision, SourceSafety};

fn safe() -> SourceSafety {
    SourceSafety {
        conversion_succeeded: true,
        output_exists: true,
        original_bytes: 100,
        output_bytes: 40,
    }
}

fn decision(policy: SourcePolicy, safety: SourceSafety) -> SourceActionDecision {
    decide_source_action(PathBuf::from("input.mov"), policy, safety)
}

fn action(policy: SourcePolicy, safety: SourceSafety) -> SourceAction {
    decision(policy, safety).action
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
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "conversion did not succeed");
}

#[test]
fn missing_output_keeps_original() {
    let mut safety = safe();
    safety.output_exists = false;
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "output file is missing");
}

#[test]
fn zero_output_keeps_original() {
    let mut safety = safe();
    safety.output_bytes = 0;
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "output size is zero");
}

#[test]
fn zero_original_keeps_original() {
    let mut safety = safe();
    safety.original_bytes = 0;
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "original size is unavailable");
}

#[test]
fn larger_output_keeps_original() {
    let mut safety = safe();
    safety.output_bytes = 120;
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "output is not smaller than original");
}

#[test]
fn equal_size_output_keeps_original() {
    let mut safety = safe();
    safety.output_bytes = safety.original_bytes;
    let decision = decision(SourcePolicy::Trash, safety);

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert_eq!(decision.reason, "output is not smaller than original");
}
