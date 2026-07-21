use super::{parse_args, parse_pids, probe_label};

#[test]
fn defaults_to_mounted_dmg_binary_and_process() {
    let (binary, process) = parse_args(Vec::new()).unwrap();
    assert!(binary.contains("/Volumes/DropSquash/DropSquash.app"));
    assert_eq!(process, "DropSquash");
}

#[test]
fn parses_optional_binary_and_process() {
    let (binary, process) = parse_args(vec!["/tmp/app".into(), "Preview".into()]).unwrap();
    assert_eq!(binary, "/tmp/app");
    assert_eq!(process, "Preview");
}

#[test]
fn returns_empty_pid_list_when_pgrep_finds_nothing() {
    assert!(parse_pids(false, b"123\n456\n").is_empty());
}

#[test]
fn parses_pid_output_lines() {
    assert_eq!(parse_pids(true, b"123\n456\n"), ["123", "456"]);
}

#[test]
fn labels_mounted_and_regular_apps() {
    assert_eq!(
        probe_label("/Volumes/DropSquash/DropSquash.app/Contents/MacOS/dropsquash-desktop"),
        "mounted dmg",
    );
    assert_eq!(
        probe_label("/tmp/dsq-build-target/release/bundle/macos/DropSquash.app/Contents/MacOS/dropsquash-desktop"),
        "app",
    );
}
