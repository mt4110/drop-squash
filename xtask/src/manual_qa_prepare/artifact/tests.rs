use super::qa_artifact;
use crate::manual_qa_prepare::options::Options;

#[test]
fn accepts_existing_app_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::create_dir(&artifact).unwrap();

    let resolved = qa_artifact(&options(Some(artifact.clone()))).unwrap();

    assert_eq!(resolved, Some(artifact));
}

#[test]
fn rejects_missing_app_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");

    let error = qa_artifact(&options(Some(artifact))).unwrap_err();

    assert!(error.contains(".app artifact"));
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
        output_dir: "/tmp/output".into(),
        reset_trial: false,
        restore_state: false,
        state_dir: "/tmp/state".into(),
    }
}
