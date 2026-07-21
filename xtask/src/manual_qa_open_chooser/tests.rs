use super::{parse_args, script_args};

#[test]
fn script_uses_sheet_check_and_reverse_tabs() {
    let script = script_args(None, None).join("\n");
    assert!(script.contains("set frontmost of process \"dropsquash-desktop\" to true"));
    assert!(script.contains("set processName to \"DropSquash\""));
    assert!(script.contains("set processName to \"dropsquash-desktop\""));
    assert!(script.contains("button \"OK\" of window 1 of process processName"));
    assert!(script.contains("click button \"Choose recording\""));
    assert!(script.contains("repeat 6 times"));
    assert!(script.contains("key code 48 using shift down"));
    assert!(script.contains("key code 49"));
    assert!(script.contains("count of sheets of window 1 of process processName"));
    assert!(script.contains("Choose a recording"));
    assert!(script.contains("key code 53"));
    assert!(script.contains("else\nkey code 48 using shift down"));
}

#[test]
fn script_can_open_panel_directory() {
    let script = script_args(Some("/tmp/dropsquash-qa-open-panel"), None).join("\n");
    assert!(script.contains("click button \"Choose recording\""));
    assert!(!script.contains("/private/tmp/dropsquash-qa-open-panel"));
}

#[test]
fn script_can_open_a_sample_path_directly() {
    let script =
        script_args(Some("/tmp/dropsquash-qa-open-panel"), Some("qa-small.mov")).join("\n");
    assert!(script.contains("click button \"Choose recording\""));
    assert!(!script.contains("qa-small.mov"));
}

#[test]
fn panel_runs_without_osascript_pre_step() {
    let (panel, sample) = parse_args(vec![
        "/tmp/dropsquash-qa-open-panel".into(),
        "qa-small.mov".into(),
    ])
    .unwrap();
    assert_eq!(panel.as_deref(), Some("/tmp/dropsquash-qa-open-panel"));
    assert_eq!(sample.as_deref(), Some("qa-small.mov"));
}
