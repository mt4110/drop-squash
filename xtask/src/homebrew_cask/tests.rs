use super::{render, Input};

const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn renders_drop_squash_cask() {
    let input = Input::parse(vec![
        "0.1.0".to_string(),
        "https://example.com/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://example.com/dropsquash".to_string(),
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
        "http://example.com/DropSquash.dmg".to_string(),
        SHA256.to_string(),
        "https://example.com/dropsquash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("https://"));
}

#[test]
fn rejects_invalid_sha256() {
    let error = Input::parse(vec![
        "0.1.0".to_string(),
        "https://example.com/DropSquash.dmg".to_string(),
        "abc".to_string(),
        "https://example.com/dropsquash".to_string(),
    ])
    .err()
    .unwrap();

    assert!(error.contains("64 hex"));
}
