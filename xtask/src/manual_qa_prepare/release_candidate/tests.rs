use super::{checksum_command, print_rows, rows};

#[test]
fn accepts_checked_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::write(&path, dmg_bytes(b"dropsquash")).unwrap();

    let rows = rows(&path).unwrap();

    assert_eq!(rows.len(), 2);
    assert!(rows[0].contains("artifact-check passed"));
    assert!(rows[0].contains(path.to_str().unwrap()));
    assert!(rows[1].contains("lowercase SHA-256"));
    assert!(rows[1].contains("SHA256SUMS"));
    assert!(rows[1].contains("DropSquash.dmg"));
    print_rows(&path, directory.path()).unwrap();
}

#[test]
fn prints_checksum_command_for_prepared_output_folder() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    let output_dir = directory.path().join("qa-output");

    let command = checksum_command(&path, &output_dir);

    assert!(command.contains("cargo run -p xtask -- checksum"));
    assert!(command.contains(path.to_str().unwrap()));
    assert!(command.contains(output_dir.join("SHA256SUMS").to_str().unwrap()));
}

#[test]
fn generated_rows_include_app_artifact_path_and_digest() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::write(&path, dmg_bytes(b"dropsquash")).unwrap();
    let digest = crate::checksum::checksum_line(&path)
        .unwrap()
        .split_once("  ")
        .unwrap()
        .0
        .to_string();

    let rows = rows(&path).unwrap().join("\n");

    assert!(rows.contains(path.to_str().unwrap()));
    assert!(rows.contains(&digest));
}

#[test]
fn rejects_dmg_with_nix_reference() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::write(&path, dmg_bytes(b"/nix/store/abc")).unwrap();

    let error = print_rows(&path, directory.path()).unwrap_err();

    assert!(error.contains("/nix/store"));
}

#[test]
fn generated_rows_satisfy_manual_qa_artifact_checks() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n{}\n",
            artifact.display(),
            rows(&artifact).unwrap().join("\n")
        ),
    )
    .unwrap();

    let missing = crate::manual_qa_check::check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("artifact-check")));
    assert!(!missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn generated_row_labels_are_required_manual_qa_checks() {
    let untracked = [super::ARTIFACT_CHECK, super::CHECKSUM]
        .into_iter()
        .filter(|label| !crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(label))
        .collect::<Vec<_>>();

    assert!(untracked.is_empty(), "{untracked:?}");
}

#[test]
fn generated_row_labels_exist_in_manual_qa_template() {
    let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();
    let missing = [super::ARTIFACT_CHECK, super::CHECKSUM]
        .into_iter()
        .filter(|label| !template.contains(&format!("| {label} |")))
        .collect::<Vec<_>>();

    assert!(missing.is_empty(), "{missing:?}");
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
