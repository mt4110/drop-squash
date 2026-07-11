use super::missing_release_workflow_gates;

#[test]
fn accepts_release_workflow_with_required_gates() {
    let missing = missing_release_workflow_gates(
        r#"
run: cargo run -p xtask -- file-size-check
run: cargo run -p xtask -- website-check
run: cargo run -p xtask -- release-check
name: Block unsigned Phase 0 release
"#,
    );

    assert!(missing.is_empty());
}

#[test]
fn reports_missing_release_workflow_gates() {
    let missing = missing_release_workflow_gates("run: cargo run -p xtask -- release-check");

    assert_eq!(
        missing,
        vec![
            "cargo run -p xtask -- file-size-check",
            "cargo run -p xtask -- website-check",
            "Block unsigned Phase 0 release"
        ]
    );
}
