use super::validate;

#[test]
fn rejects_relative_artifact_path() {
    let mut missing = Vec::new();

    validate("target/release/bundle/macos/DropSquash.app", &mut missing);

    assert!(missing.iter().any(|error| error.contains("absolute path")));
}
