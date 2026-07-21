use super::check_runtime_guards;

#[test]
fn runtime_guards_match_repository_sources() {
    assert!(check_runtime_guards().is_ok());
}

#[test]
fn fails_when_runtime_marker_disappears() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("App.tsx");
    std::fs::write(&path, "useWindowHeight(shellRef, [state.isPro]);").unwrap();
    let error = super::require_markers(
        &path,
        &[
            "useWindowHeight(shellRef, [",
            "state.isPro",
            "applicationsInstall.shouldShowNotice",
        ],
        "desktop window-height hook wiring",
    )
    .unwrap_err();

    assert!(error.contains("desktop window-height hook wiring"));
    assert!(error.contains("applicationsInstall.shouldShowNotice"));
}
