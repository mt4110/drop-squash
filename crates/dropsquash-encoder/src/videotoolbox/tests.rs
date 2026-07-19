use super::*;
use dropsquash_core::{
    mask_options_for_frame, Confidence, EncodeJob, FrameMaskPlan, FrameSize, FrameStatus, MaskMode,
    MaskPlan, MaskPlanAudit, MaskPolicy, MaskReason, MaskRect, MaskRegion, ObservationSource,
    OutputSize, PixelRect, Profile, RegionPolicy, SecureShareOptions, SourcePolicy,
    VerificationExpectations,
};
use objc2_av_foundation::{
    AVAssetExportPreset1280x720, AVAssetExportPreset640x480, AVAssetExportPreset960x540,
    AVAssetExportPresetLowQuality, AVAssetExportPresetMediumQuality,
};

fn job_in(directory: &tempfile::TempDir) -> EncodeJob {
    EncodeJob {
        input_path: directory.path().join("recording.mov"),
        output_dir: directory.path().to_path_buf(),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Ask,
        secure_share: None,
    }
}

#[test]
fn secure_share_missing_input_fails_before_native_bootstrap() {
    let directory = tempfile::tempdir().unwrap();
    let mut job = job_in(&directory);
    job.secure_share = Some(SecureShareOptions {
        mask_mode: MaskMode::SolidBlack,
        mask_rects: vec![MaskRect {
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        }],
    });

    let error =
        encode::encode_with_avfoundation(job, None, tokio_util::sync::CancellationToken::new())
            .unwrap_err();

    assert!(error.to_string().contains("file does not exist"));
}

#[test]
fn destructive_mask_overwrites_only_target_rect() {
    let mut frame = vec![200u8; 4 * 4 * 4];
    mask::apply_destructive_mask_rgba(
        &mut frame,
        4,
        4,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: vec![MaskRect {
                x: 1,
                y: 1,
                width: 2,
                height: 2,
            }],
        },
        7,
    )
    .unwrap();

    assert_eq!(rgba_at(&frame, 4, 0, 0), [200, 200, 200, 200]);
    assert_eq!(rgba_at(&frame, 4, 1, 1), [0, 0, 0, 255]);
    assert_eq!(rgba_at(&frame, 4, 2, 2), [0, 0, 0, 255]);
    assert_eq!(rgba_at(&frame, 4, 3, 3), [200, 200, 200, 200]);
}

#[test]
fn mask_plan_derived_options_blacken_pixels() {
    let plan = mask_plan_with_region(MaskRegion {
        rect: PixelRect {
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        },
        policy: RegionPolicy::Sensitive,
        reason: MaskReason::VisionText,
        sources: vec![ObservationSource::VisionTextRecognition],
        confidence: Confidence::CERTAIN,
        expansion_px: 0,
    });
    let options = mask_options_for_frame(&plan, 0, MaskMode::SolidBlack).unwrap();
    let mut frame = vec![200u8; 4 * 4 * 4];

    mask::apply_destructive_mask_rgba(&mut frame, 4, 4, &options, 0).unwrap();

    assert_eq!(rgba_at(&frame, 4, 0, 0), [200, 200, 200, 200]);
    assert_eq!(rgba_at(&frame, 4, 1, 1), [0, 0, 0, 255]);
    assert_eq!(rgba_at(&frame, 4, 2, 2), [0, 0, 0, 255]);
}

#[test]
fn black_noise_mask_is_destructive_and_deterministic() {
    let mut frame = vec![180u8; 3 * 2 * 4];
    mask::apply_destructive_mask_rgba(
        &mut frame,
        3,
        2,
        &SecureShareOptions {
            mask_mode: MaskMode::BlackNoise,
            mask_rects: vec![MaskRect {
                x: 0,
                y: 0,
                width: 2,
                height: 1,
            }],
        },
        99,
    )
    .unwrap();

    let first = rgba_at(&frame, 3, 0, 0);
    let second = rgba_at(&frame, 3, 1, 0);
    assert_ne!(first, [180, 180, 180, 180]);
    assert_ne!(second, [180, 180, 180, 180]);
    assert!(first[0] <= 63 && first[1] <= 63 && first[2] <= 63);
    assert_eq!(first[3], 255);
    assert_eq!(rgba_at(&frame, 3, 2, 1), [180, 180, 180, 180]);
}

#[test]
fn destructive_mask_rejects_invalid_frame_shape() {
    let mut invalid_frame = [0u8; 7];
    let error = mask::apply_destructive_mask_rgba(
        &mut invalid_frame,
        1,
        2,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: vec![MaskRect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            }],
        },
        0,
    )
    .unwrap_err();

    assert!(error.to_string().contains("frame shape mismatch"));
}

#[test]
fn destructive_mask_supports_padded_rows() {
    let mut frame = vec![180u8; 2 * 20];
    mask::apply_destructive_mask_rgba_stride(
        &mut frame,
        3,
        2,
        20,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: vec![MaskRect {
                x: 1,
                y: 0,
                width: 2,
                height: 2,
            }],
        },
        0,
    )
    .unwrap();

    assert_eq!(&frame[0..4], &[180, 180, 180, 180]);
    assert_eq!(&frame[4..8], &[0, 0, 0, 255]);
    assert_eq!(&frame[8..12], &[0, 0, 0, 255]);
    assert_eq!(&frame[12..20], &[180; 8]);
    assert_eq!(&frame[20..24], &[180, 180, 180, 180]);
    assert_eq!(&frame[24..28], &[0, 0, 0, 255]);
    assert_eq!(&frame[28..32], &[0, 0, 0, 255]);
    assert_eq!(&frame[32..40], &[180; 8]);
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

    assert!(presets::presets_for(&job).is_err());
}

#[test]
fn explicit_output_size_selects_a_native_preset() {
    let mut job = job_in(&tempfile::tempdir().unwrap());

    job.output_size = OutputSize::P1080;
    assert_eq!(
        preset_names(&presets::presets_for(&job).unwrap()),
        preset_names(&unsafe {
            vec![
                objc2_av_foundation::AVAssetExportPreset1920x1080,
                AVAssetExportPreset1280x720,
                AVAssetExportPreset960x540,
                AVAssetExportPreset640x480,
                AVAssetExportPresetMediumQuality,
                AVAssetExportPresetLowQuality,
            ]
        })
    );
    job.output_size = OutputSize::P720;
    assert_eq!(
        preset_names(&presets::presets_for(&job).unwrap()),
        preset_names(&unsafe {
            vec![
                AVAssetExportPreset1280x720,
                AVAssetExportPreset960x540,
                AVAssetExportPreset640x480,
                AVAssetExportPresetMediumQuality,
                AVAssetExportPresetLowQuality,
            ]
        })
    );
    job.output_size = OutputSize::P480;
    assert_eq!(
        preset_names(&presets::presets_for(&job).unwrap()),
        preset_names(&unsafe {
            vec![
                AVAssetExportPreset640x480,
                AVAssetExportPresetMediumQuality,
                AVAssetExportPresetLowQuality,
            ]
        })
    );
}

#[test]
fn auto_size_uses_a_fallback_chain_for_docs_like_profiles() {
    let directory = tempfile::tempdir().unwrap();
    let job = job_in(&directory);
    std::fs::write(&job.input_path, []).unwrap();

    let presets = presets::presets_for(&job).unwrap();

    assert_eq!(
        preset_names(&presets),
        preset_names(&unsafe {
            vec![
                AVAssetExportPreset1280x720,
                AVAssetExportPreset960x540,
                AVAssetExportPreset640x480,
                AVAssetExportPresetMediumQuality,
                AVAssetExportPresetLowQuality,
            ]
        })
    );
}

#[test]
fn explicit_profile_with_auto_size_uses_a_smaller_fallback_chain() {
    let directory = tempfile::tempdir().unwrap();
    let mut job = job_in(&directory);
    job.profile = Profile::Slack;
    std::fs::write(&job.input_path, []).unwrap();

    let presets = presets::presets_for(&job).unwrap();

    assert_eq!(
        preset_names(&presets),
        preset_names(&unsafe {
            vec![
                AVAssetExportPreset960x540,
                AVAssetExportPreset640x480,
                AVAssetExportPresetMediumQuality,
                AVAssetExportPresetLowQuality,
            ]
        })
    );
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

#[test]
fn not_smaller_verification_error_stays_friendly() {
    let error = encode::verification_error(Some(crate::OutputVerification {
        output_exists: true,
        output_bytes: 1235958,
        original_bytes: 1177311,
        output_extension_is_mp4: true,
        has_mp4_file_type: true,
        has_nonzero_duration: true,
        duration_matches_source: true,
        is_smaller_than_original: false,
        is_valid_output: false,
    }));

    assert_eq!(
        error.to_string(),
        "encoder failed: This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn not_smaller_with_other_verification_failures_stays_friendly() {
    let error = encode::verification_error(Some(crate::OutputVerification {
        output_exists: true,
        output_bytes: 1235958,
        original_bytes: 1177311,
        output_extension_is_mp4: true,
        has_mp4_file_type: false,
        has_nonzero_duration: true,
        duration_matches_source: true,
        is_smaller_than_original: false,
        is_valid_output: false,
    }));

    assert_eq!(
        error.to_string(),
        "encoder failed: This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

fn preset_names(presets: &[&objc2_foundation::NSString]) -> Vec<String> {
    presets.iter().map(|preset| preset.to_string()).collect()
}

fn rgba_at(frame: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let offset = ((y * width + x) * 4) as usize;
    [
        frame[offset],
        frame[offset + 1],
        frame[offset + 2],
        frame[offset + 3],
    ]
}

fn mask_plan_with_region(region: MaskRegion) -> MaskPlan {
    MaskPlan {
        schema_version: 1,
        capture_id: "encoder-mask-plan-proof".to_string(),
        frame_size: FrameSize {
            width: 4,
            height: 4,
        },
        frames: vec![FrameMaskPlan {
            frame_index: 0,
            presentation_time_ns: 0,
            frame_status: FrameStatus::Complete,
            regions: vec![region],
        }],
        policy: MaskPolicy::StrictReveal,
        audit: MaskPlanAudit::clean(),
        verification_expectations: VerificationExpectations {
            no_audio: true,
            strip_metadata: true,
            verification_policy_version: "alpha-black-fill-proof".to_string(),
        },
    }
}
