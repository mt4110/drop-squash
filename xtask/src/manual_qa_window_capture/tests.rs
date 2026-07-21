use super::{bounds::parse as parse_bounds, parse_args, resolved_output};

#[test]
fn defaults_to_drop_squash_window_capture() {
    let (output, process) = parse_args(vec![]).unwrap();
    assert_eq!(output, "/tmp/dropsquash-window.png");
    assert_eq!(process, "dropsquash-desktop");
}

#[test]
fn accepts_custom_output_and_process() {
    let (output, process) = parse_args(vec!["/tmp/shot.png".into(), "OtherApp".into()]).unwrap();
    assert_eq!(output, "/tmp/shot.png");
    assert_eq!(process, "OtherApp");
}

#[test]
fn rejects_bad_bounds() {
    assert!(parse_bounds("1,2,320,240").is_ok());
    assert!(parse_bounds("1,2,0,240").is_err());
    assert!(parse_bounds("1,2,320").is_err());
}

#[test]
fn normalizes_tmp_output_to_private_tmp() {
    assert_eq!(resolved_output("/tmp/shot.png"), "/private/tmp/shot.png",);
}
