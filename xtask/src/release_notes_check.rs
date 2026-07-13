use std::path::{Path, PathBuf};

mod release_notes_evidence;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let [path] = args.as_slice() else {
        return Err("release-notes-check requires exactly <release-notes.md>".to_string());
    };
    check_file(&PathBuf::from(path))
}

pub(crate) fn check_file(path: &Path) -> Result<(), String> {
    check_file_silent(path)?;
    println!("release notes checks passed");
    Ok(())
}

pub(crate) fn check_file_silent(path: &Path) -> Result<(), String> {
    let errors = release_notes_evidence::check(path)?;
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn rejects_missing_release_notes_argument() {
        let error = run(Vec::new()).unwrap_err();

        assert!(error.contains("exactly <release-notes.md>"));
    }

    #[test]
    fn rejects_extra_release_notes_arguments() {
        let error = run(vec!["notes.md".into(), "extra.md".into()]).unwrap_err();

        assert!(error.contains("exactly <release-notes.md>"));
    }
}
