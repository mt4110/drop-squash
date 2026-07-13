use std::time::Duration;

use super::inspect_mp4;

#[test]
fn detects_mp4_file_type_and_duration() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sample.mp4");
    std::fs::write(&path, fixture_mp4(600, 1_200)).unwrap();

    let inspection = inspect_mp4(&path);

    assert!(inspection.has_file_type);
    assert_eq!(inspection.duration, Some(Duration::from_secs(2)));
    assert!(inspection.has_nonzero_duration());
}

#[test]
fn skips_empty_top_level_boxes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sample.mov");
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x14ftypqt  \0\0\0\0qt  ");
    bytes.extend_from_slice(b"\0\0\0\x08wide");
    bytes.extend_from_slice(&fixture_moov(600, 1_200));
    std::fs::write(&path, bytes).unwrap();

    let inspection = inspect_mp4(&path);

    assert!(inspection.has_file_type);
    assert_eq!(inspection.duration, Some(Duration::from_secs(2)));
}

#[test]
fn reads_version_one_movie_header_duration() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sample.mov");
    std::fs::write(&path, fixture_mp4_v1(600, 1_200)).unwrap();

    let inspection = inspect_mp4(&path);

    assert!(inspection.has_file_type);
    assert_eq!(inspection.duration, Some(Duration::from_secs(2)));
}

#[test]
fn rejects_zero_duration() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sample.mp4");
    std::fs::write(&path, fixture_mp4(600, 0)).unwrap();

    let inspection = inspect_mp4(&path);

    assert!(inspection.has_file_type);
    assert_eq!(inspection.duration, Some(Duration::ZERO));
    assert!(!inspection.has_nonzero_duration());
}

#[test]
fn ignores_invalid_timescale() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sample.mp4");
    std::fs::write(&path, fixture_mp4(0, 1_200)).unwrap();

    let inspection = inspect_mp4(&path);

    assert!(inspection.has_file_type);
    assert_eq!(inspection.duration, None);
    assert!(!inspection.has_nonzero_duration());
}

fn fixture_mp4(timescale: u32, duration: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x14ftypmp42\0\0\0\0mp42");
    bytes.extend_from_slice(&fixture_moov(timescale, duration));
    bytes
}

fn fixture_moov(timescale: u32, duration: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x24moov");
    bytes.extend_from_slice(&fixture_mvhd_v0(timescale, duration));
    bytes
}

fn fixture_mvhd_v0(timescale: u32, duration: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x1cmvhd");
    bytes.extend_from_slice(&[0, 0, 0, 0]);
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(&timescale.to_be_bytes());
    bytes.extend_from_slice(&duration.to_be_bytes());
    bytes
}

fn fixture_mp4_v1(timescale: u32, duration: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\0\0\0\x14ftypmp42\0\0\0\0mp42");
    bytes.extend_from_slice(b"\0\0\0\x30moov");
    bytes.extend_from_slice(b"\0\0\0\x28mvhd");
    bytes.extend_from_slice(&[1, 0, 0, 0]);
    bytes.extend_from_slice(&[0; 16]);
    bytes.extend_from_slice(&timescale.to_be_bytes());
    bytes.extend_from_slice(&duration.to_be_bytes());
    bytes
}
