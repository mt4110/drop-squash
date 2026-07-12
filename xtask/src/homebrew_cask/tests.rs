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
    assert!(cask.contains("auto_updates false"));
    assert!(cask.contains("zap trash: \"~/Library/Application Support/DropSquash\""));
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

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_wrong_dmg_name() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/Other.dmg".to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("DropSquash.dmg"));
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
fn rejects_imposter_github_release_host() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com.evil/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg"
            .to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("GitHub Release"));
}

#[test]
fn rejects_url_with_inline_note() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg TBD"
            .to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("whitespace"));
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
fn rejects_nested_github_release_asset_url() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/nested/DropSquash.dmg"
            .to_string(),
        SHA256.to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("GitHub Release"));
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

    assert!(error.contains("64-character hex"));
}

#[test]
fn rejects_placeholder_sha256() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".to_string(),
        "0".repeat(64),
        "https://github.com/mt4110/drop-squash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("real"));
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
