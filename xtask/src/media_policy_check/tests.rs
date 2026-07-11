use super::check_roots;

#[test]
fn accepts_clean_product_sources() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "apps/app.rs", "fn main() {}");

    assert!(check_roots(&[directory.path().join("apps")]).is_ok());
}

#[test]
fn rejects_external_media_executable_names() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "crates/media/src/lib.rs", "\"ffmpeg\"");

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("ffmpeg"));
}

#[test]
fn rejects_process_command_construction() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/cli/src/run.rs",
        "std::process::Command::new(\"tool\")",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("std::process::Command"));
}

#[test]
fn skips_generated_tauri_schema() {
    let directory = tempfile::tempdir().unwrap();
    write(directory.path(), "apps/desktop/gen/schema.json", "ffprobe");

    assert!(check_roots(&[directory.path().join("apps")]).is_ok());
}

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
