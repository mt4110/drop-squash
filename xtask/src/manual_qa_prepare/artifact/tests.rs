use super::qa_artifact;
use crate::manual_qa_prepare::options::Options;

#[test]
fn accepts_existing_app_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::create_dir(&artifact).unwrap();

    let resolved = qa_artifact(&options(Some(artifact.clone()))).unwrap();

    assert_eq!(resolved, Some(artifact.canonicalize().unwrap()));
}

#[test]
fn resolves_app_artifact_to_canonical_path() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::create_dir(&artifact).unwrap();
    let dotted_artifact = directory.path().join(".").join("DropSquash.app");

    let resolved = qa_artifact(&options(Some(dotted_artifact))).unwrap();

    assert_eq!(resolved, Some(artifact.canonicalize().unwrap()));
}

#[test]
fn rejects_missing_app_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");

    let error = qa_artifact(&options(Some(artifact))).unwrap_err();

    assert!(error.contains("DropSquash.app"));
}

#[test]
fn rejects_noncanonical_app_artifact_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.app");
    std::fs::create_dir(&artifact).unwrap();

    let error = qa_artifact(&options(Some(artifact))).unwrap_err();

    assert!(error.contains("DropSquash.app"));
}

#[test]
fn rejects_noncanonical_dmg_artifact_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();

    let error = qa_artifact(&options(Some(artifact))).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn ignores_missing_default_artifact() {
    let resolved = qa_artifact(&options(None)).unwrap();

    assert!(resolved.is_none() || resolved.unwrap().ends_with("DropSquash.app"));
}

fn options(app_artifact: Option<std::path::PathBuf>) -> Options {
    Options {
        app_artifact,
        app_state_dir: "/tmp/app-state".into(),
        input_sample_set: None,
        markdown_output: None,
        output_dir: "/tmp/output".into(),
        reset_trial: false,
        restore_state: false,
        state_dir: "/tmp/state".into(),
    }
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
