use std::path::{Path, PathBuf};

mod release_notes_evidence;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = args
        .first()
        .ok_or_else(|| "release-notes-check requires <release-notes.md>".to_string())?;
    check_file(&PathBuf::from(path))
}

pub(crate) fn check_file(path: &Path) -> Result<(), String> {
    let errors = release_notes_evidence::check(path)?;
    if errors.is_empty() {
        println!("release notes checks passed");
        return Ok(());
    }
    Err(errors.join("\n"))
}
