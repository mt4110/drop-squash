use super::{render, Input};

const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn renders_drop_squash_cask() {
    let input = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .unwrap();

    let cask = render(&input);

    assert!(cask.contains("cask \"dropsquash\" do"));
    assert!(cask.contains("version \"0.1.0\""));
    assert!(cask.contains("app \"DropSquash.app\""));
}

#[test]
fn rejects_non_https_url() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "http://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("https://"));
}

#[test]
fn rejects_v_prefixed_version() {
    let error = Input::parse(vec![
        "v0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("major.minor.patch"));
}

#[test]
fn rejects_partial_version() {
    let error = Input::parse(vec![
        "0.1".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("major.minor.patch"));
}

#[test]
fn rejects_example_dot_com_url() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://example.com/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("example.com"));
}

#[test]
fn rejects_non_dmg_url() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.zip".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains(".dmg"));
}

#[test]
fn rejects_non_github_release_url() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://downloads.example.test/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("GitHub Release"));
}

#[test]
fn rejects_url_for_different_version() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.2.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("matching"));
}

#[test]
fn rejects_invalid_sha256() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        "abc".to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("64 hex"));
}

#[test]
fn rejects_non_canonical_homepage() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://dropsquash.app".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("canonical DropSquash repository"));
}
