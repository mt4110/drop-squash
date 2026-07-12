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

    assert!(error.contains("std::process::command"));
}

#[test]
fn rejects_path_lookup_helpers() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/media/src/path.rs",
        r#"std::env::var("PATH"); which::which("ffmpeg");"#,
    );

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("std::env::var(\"path\")"));
    assert!(error.contains("which::which"));
}

#[test]
fn rejects_javascript_media_process_packages() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "apps/desktop/web/src/media.ts",
        "import cp from 'child_process'; import ffmpeg from 'fluent-ffmpeg';",
    );
    write(
        directory.path(),
        "apps/desktop/web/src/wasm.ts",
        "import { FFmpeg } from '@ffmpeg/ffmpeg';",
    );

    let error = check_roots(&[directory.path().join("apps")]).unwrap_err();

    assert!(error.contains("child_process"));
    assert!(error.contains("fluent-ffmpeg"));
    assert!(error.contains("@ffmpeg/"));
}

#[test]
fn rejects_other_external_media_tools() {
    let directory = tempfile::tempdir().unwrap();
    write(
        directory.path(),
        "crates/media/src/lib.rs",
        "\"mediainfo\"; \"avconv\";",
    );

    let error = check_roots(&[directory.path().join("crates")]).unwrap_err();

    assert!(error.contains("\"mediainfo\""));
    assert!(error.contains("avconv"));
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
