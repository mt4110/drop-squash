use std::io::Write;

use super::check_root;

#[test]
fn accepts_files_within_limits() {
    let directory = tempfile::tempdir().unwrap();
    write_lines(&directory.path().join("src/lib.rs"), 128);
    write_lines(&directory.path().join("web/app.tsx"), 512);

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_large_rust_production_file() {
    let directory = tempfile::tempdir().unwrap();
    write_lines(&directory.path().join("src/lib.rs"), 129);

    let violations = check_root(directory.path()).unwrap();

    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("limit is 128"));
}

#[test]
fn ignores_rust_test_files() {
    let directory = tempfile::tempdir().unwrap();
    write_lines(&directory.path().join("src/tests.rs"), 1001);
    write_lines(&directory.path().join("tests/integration.rs"), 1001);

    assert!(check_root(directory.path()).unwrap().is_empty());
}

#[test]
fn rejects_large_typescript_file() {
    let directory = tempfile::tempdir().unwrap();
    write_lines(&directory.path().join("app.ts"), 513);

    let violations = check_root(directory.path()).unwrap();

    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("limit is 512"));
}

#[test]
fn skips_generated_and_private_directories() {
    let directory = tempfile::tempdir().unwrap();
    write_lines(&directory.path().join("target/generated.rs"), 999);
    write_lines(&directory.path().join(".private_docs/private.ts"), 999);
    write_lines(&directory.path().join("node_modules/pkg/index.ts"), 999);

    assert!(check_root(directory.path()).unwrap().is_empty());
}

fn write_lines(path: &std::path::Path, count: usize) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = std::fs::File::create(path).unwrap();
    for _ in 0..count {
        writeln!(file, "line").unwrap();
    }
}
