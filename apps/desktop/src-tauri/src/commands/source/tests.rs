use dropsquash_core::{EncodeResult, Profile, SourcePolicy};
use dropsquash_postprocess::SourceAction;

use super::{handle_source_action, trash_original};

#[test]
fn ask_policy_does_not_move_original_without_confirmation() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("recording.mov");
    let output = directory.path().join("recording.squashed.mp4");
    std::fs::write(&source, vec![0; 100]).unwrap();
    std::fs::write(&output, vec![0; 20]).unwrap();

    let decision =
        handle_source_action(&result(source.clone(), output), SourcePolicy::Ask).unwrap();

    assert_eq!(decision.action, SourceAction::AskUser);
    assert!(source.exists());
}

#[test]
fn explicit_trash_revalidates_output_before_moving_original() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("recording.mov");
    let output = directory.path().join("recording.squashed.mp4");
    std::fs::write(&source, vec![0; 100]).unwrap();
    std::fs::write(&output, b"not an mp4").unwrap();

    let decision = trash_original(
        source.to_string_lossy().into_owned(),
        output.to_string_lossy().into_owned(),
    )
    .unwrap();

    assert_eq!(decision.action, SourceAction::KeepOriginal);
    assert!(source.exists());
}

fn result(input_path: std::path::PathBuf, output_path: std::path::PathBuf) -> EncodeResult {
    EncodeResult {
        input_path,
        output_path,
        profile: Profile::Auto,
        original_bytes: 100,
        output_bytes: 20,
        success: true,
        error_message: None,
    }
}
