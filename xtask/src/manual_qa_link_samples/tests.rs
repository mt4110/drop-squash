use super::create_links;

#[test]
fn creates_ascii_links_for_three_samples() {
    let temp = tempfile::tempdir().unwrap();
    let source = tempfile::tempdir().unwrap();
    let small = source.path().join("画面収録.mov");
    let medium = source.path().join("medium.mov");
    let large = source.path().join("large.mp4");
    std::fs::write(&small, "").unwrap();
    std::fs::write(&medium, "").unwrap();
    std::fs::write(&large, "").unwrap();
    let rows = vec![
        vec!["backend".into(), "input".into()],
        vec!["apple-native".into(), small.display().to_string()],
        vec!["apple-native".into(), medium.display().to_string()],
        vec!["apple-native".into(), large.display().to_string()],
    ];

    let links = create_links(&rows, temp.path()).unwrap();

    assert_eq!(links.len(), 3);
    assert_eq!(std::fs::read_link(&links[0]).unwrap(), small);
    assert_eq!(std::fs::read_link(&links[1]).unwrap(), medium);
    assert_eq!(std::fs::read_link(&links[2]).unwrap(), large);
}
