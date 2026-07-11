use super::*;
use dropsquash_core::{EncodeJob, OutputSize, Profile, SourcePolicy};

fn job_in(directory: &tempfile::TempDir) -> EncodeJob {
    EncodeJob {
        input_path: directory.path().join("recording.mov"),
        output_dir: directory.path().to_path_buf(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
    }
}

#[test]
fn avfoundation_backend_reports_available() {
    let capabilities = AppleNativeEncoder.probe_capabilities().unwrap();

    assert_eq!(capabilities.backend_name, "apple-native");
    assert!(capabilities.available);
    assert!(capabilities.supports_h264);
    assert!(!capabilities.hardware_acceleration);
}

#[test]
fn privacy_profile_does_not_select_a_lossy_preset() {
    let mut job = job_in(&tempfile::tempdir().unwrap());
    job.profile = Profile::Privacy;

    assert!(presets::preset_for(&job).is_err());
}

#[test]
fn explicit_output_size_selects_a_native_preset() {
    let mut job = job_in(&tempfile::tempdir().unwrap());

    job.output_size = OutputSize::P1080;
    assert!(presets::preset_for(&job).is_ok());
    job.output_size = OutputSize::P720;
    assert!(presets::preset_for(&job).is_ok());
    job.output_size = OutputSize::P480;
    assert!(presets::preset_for(&job).is_ok());
}

#[test]
fn preserves_existing_output_by_selecting_a_numbered_path() {
    let directory = tempfile::tempdir().unwrap();
    let job = job_in(&directory);
    std::fs::write(directory.path().join("recording.squashed.mp4"), b"existing").unwrap();

    assert_eq!(
        paths::output_path_for(&job).unwrap(),
        directory.path().join("recording.squashed-2.mp4")
    );
}

#[test]
fn finalization_moves_temp_output_to_final_path() {
    let directory = tempfile::tempdir().unwrap();
    let temporary_output = directory.path().join("output.tmp.mp4");
    let final_output = directory.path().join("output.mp4");
    std::fs::write(&temporary_output, b"squashed").unwrap();

    finalize::finalize_verified_output(&temporary_output, &final_output).unwrap();

    assert!(!temporary_output.exists());
    assert_eq!(std::fs::read(&final_output).unwrap(), b"squashed");
}

#[test]
fn finalization_refuses_to_overwrite_existing_output() {
    let directory = tempfile::tempdir().unwrap();
    let temporary_output = directory.path().join("output.tmp.mp4");
    let final_output = directory.path().join("output.mp4");
    std::fs::write(&temporary_output, b"new").unwrap();
    std::fs::write(&final_output, b"existing").unwrap();

    assert!(finalize::finalize_verified_output(&temporary_output, &final_output).is_err());
    assert_eq!(std::fs::read(&final_output).unwrap(), b"existing");
    assert_eq!(std::fs::read(&temporary_output).unwrap(), b"new");
}
