use dropsquash_core::{CaptureFrameMetadata, CaptureRect, FrameStatus};
use dropsquash_platform::NativeDestructionEvidence;

use super::super::super::super::paths::RecordingPaths;
use super::super::super::super::selection::NativeWindowSelection;
use super::{strict_shield_evidence, ReportInput};

#[test]
fn accepts_one_strict_shield_fact_per_complete_frame() {
    let directory = tempfile::tempdir().unwrap();
    let frames = vec![frame(0), frame(1)];
    let destruction = vec![evidence(0), evidence(1)];
    let paths = paths(&directory);

    let audit = strict_shield_evidence(&input(&paths, &frames, &destruction)).unwrap();

    assert_eq!(audit.masked_frame_count, 2);
    assert_eq!(audit.masked_rect_count, 2);
    assert_eq!(audit.verified_pixel_count, 2);
}

#[test]
fn rejects_missing_or_mismatched_native_destruction_facts() {
    let directory = tempfile::tempdir().unwrap();
    let frames = vec![frame(0), frame(1)];
    let paths = paths(&directory);
    let cases = [
        vec![evidence(0)],
        vec![evidence(1), evidence(0)],
        vec![
            NativeDestructionEvidence {
                policy: 2,
                ..evidence(0)
            },
            evidence(1),
        ],
        vec![
            NativeDestructionEvidence {
                output_width: 641,
                ..evidence(0)
            },
            evidence(1),
        ],
    ];

    for destruction in cases {
        assert!(strict_shield_evidence(&input(&paths, &frames, &destruction)).is_err());
    }
}

fn input<'a>(
    paths: &'a RecordingPaths,
    frames: &'a [CaptureFrameMetadata],
    destruction: &'a [NativeDestructionEvidence],
) -> ReportInput<'a> {
    ReportInput {
        selection: NativeWindowSelection {
            window_id: 1,
            owner_pid: 2,
            frame: rect(),
        },
        width: 640,
        height: 480,
        frames: frames.to_vec(),
        accessibility: Vec::new(),
        vision: Vec::new(),
        temporal: Vec::new(),
        destruction: destruction.to_vec(),
        paths,
    }
}

fn paths(directory: &tempfile::TempDir) -> RecordingPaths {
    RecordingPaths {
        partial: directory.path().join("partial.mp4"),
        final_path: directory.path().join("final.mp4"),
    }
}

fn frame(index: u64) -> CaptureFrameMetadata {
    CaptureFrameMetadata {
        frame_index: index,
        presentation_time_ns: index * 33_333_333,
        frame_status: FrameStatus::Complete,
        content_rect: rect(),
        bounding_rect: rect(),
        scale_factor: 1.0,
        content_scale: 1.0,
    }
}

fn evidence(frame_index: u64) -> NativeDestructionEvidence {
    NativeDestructionEvidence {
        frame_index,
        policy: 1,
        region_count: 1,
        output_width: 640,
        output_height: 480,
    }
}

fn rect() -> CaptureRect {
    CaptureRect {
        x: 0,
        y: 0,
        width: 640,
        height: 480,
    }
}
