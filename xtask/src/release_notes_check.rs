use std::path::PathBuf;

mod release_notes_evidence;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = args
        .first()
        .ok_or_else(|| "release-notes-check requires <release-notes.md>".to_string())?;
    let errors = release_notes_evidence::check(&PathBuf::from(path))?;
    if errors.is_empty() {
        println!("release notes checks passed");
        return Ok(());
    }
    Err(errors.join("\n"))
}
