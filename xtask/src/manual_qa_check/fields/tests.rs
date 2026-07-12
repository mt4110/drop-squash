use super::validate;

#[test]
fn rejects_relative_output_folder() {
    let mut missing = Vec::new();

    validate(
        "Output folder",
        "tmp/dropsquash-manual-qa-output",
        &mut missing,
    );

    assert!(missing.iter().any(|error| error.contains("absolute path")));
}
