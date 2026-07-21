use super::{create_links, ensure_present, release_set_rules};

#[test]
fn creates_ascii_links_for_three_samples() {
    let temp = tempfile::tempdir().unwrap();
    let source = tempfile::tempdir().unwrap();
    let small = source.path().join("画面収録.mov");
    let small_output = source.path().join("画面収録.squashed.mp4");
    let medium = source.path().join("medium.mov");
    let large = source.path().join("large.mp4");
    std::fs::write(&small, "").unwrap();
    std::fs::write(&small_output, "").unwrap();
    std::fs::write(&medium, "").unwrap();
    std::fs::write(&large, "").unwrap();
    let rows = vec![
        vec!["backend".into(), "input".into(), "output".into()],
        vec![
            "apple-native".into(),
            small.display().to_string(),
            small_output.display().to_string(),
        ],
        vec![
            "apple-native".into(),
            medium.display().to_string(),
            source
                .path()
                .join("medium.squashed.mp4")
                .display()
                .to_string(),
        ],
        vec![
            "apple-native".into(),
            large.display().to_string(),
            source
                .path()
                .join("large.squashed.mp4")
                .display()
                .to_string(),
        ],
    ];

    let links = create_links(&rows, temp.path()).unwrap();

    assert_eq!(links.len(), 4);
    assert_eq!(std::fs::read_link(&links[0]).unwrap(), small);
    assert_eq!(std::fs::read_link(&links[1]).unwrap(), medium);
    assert_eq!(std::fs::read_link(&links[2]).unwrap(), large);
    assert_eq!(std::fs::read_link(&links[3]).unwrap(), small_output);
}

#[test]
fn prints_release_set_rules_for_follow_up_manual_qa() {
    let rules = release_set_rules();

    assert_eq!(rules.len(), 4);
    assert!(rules[0].contains("original local recordings"));
    assert!(rules[1].contains("shipping setting"));
    assert!(rules[2].contains("nearly static"));
    assert!(rules[3].contains("candidate alias"));
}

#[test]
fn replaces_broken_sample_symlinks() {
    let temp = tempfile::tempdir().unwrap();
    let source = tempfile::tempdir().unwrap();
    let small = source.path().join("small.mov");
    let small_output = source.path().join("small.squashed.mp4");
    let medium = source.path().join("medium.mov");
    let large = source.path().join("large.mp4");
    let broken = source.path().join("missing.mov");
    std::fs::write(&small, "").unwrap();
    std::fs::write(&small_output, "").unwrap();
    std::fs::write(&medium, "").unwrap();
    std::fs::write(&large, "").unwrap();
    std::os::unix::fs::symlink(&broken, temp.path().join("qa-small.mov")).unwrap();
    let rows = vec![
        vec!["backend".into(), "input".into(), "output".into()],
        vec![
            "apple-native".into(),
            small.display().to_string(),
            small_output.display().to_string(),
        ],
        vec![
            "apple-native".into(),
            medium.display().to_string(),
            source
                .path()
                .join("medium.squashed.mp4")
                .display()
                .to_string(),
        ],
        vec![
            "apple-native".into(),
            large.display().to_string(),
            source
                .path()
                .join("large.squashed.mp4")
                .display()
                .to_string(),
        ],
    ];

    let links = create_links(&rows, temp.path()).unwrap();

    assert_eq!(std::fs::read_link(&links[0]).unwrap(), small);
}

#[test]
fn missing_csv_prints_recovery_steps() {
    let error = ensure_present(std::path::Path::new("/tmp/does-not-exist.csv")).unwrap_err();

    assert!(error.contains("benchmark CSV does not exist"));
    assert!(error.contains("manual-qa-prepare"));
    assert!(error.contains("benchmark-csv-check"));
}
