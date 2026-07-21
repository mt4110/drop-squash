use super::lines;

#[test]
fn prints_distribution_runbook_and_next_steps() {
    let groups = vec![(
        "Distribution And Signing",
        vec![
            (
                "`cargo run -p xtask -- macos-signing-check`".to_string(),
                String::new(),
            ),
            ("Codesign verification".to_string(), String::new()),
            (
                "Notarization staple verification".to_string(),
                String::new(),
            ),
            ("Gatekeeper open test".to_string(), String::new()),
        ],
    )];
    let lines = lines(&groups);
    for expected in [
        "distribution runbook",
        "distribution next step 1",
        "distribution next step 2",
        "distribution next step 3",
        "distribution signing reminder",
        "distribution isolated target hint",
        "distribution codesign reminder",
        "distribution notarization reminder",
        "distribution Gatekeeper reminder",
    ] {
        assert!(lines.iter().any(|line| line.contains(expected)));
    }
}
