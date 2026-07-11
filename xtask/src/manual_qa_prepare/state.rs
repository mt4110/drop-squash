use super::options::Options;

pub(super) const RESET_FILES: [&str; 2] = ["history.jsonl", "license.json"];
const STATE_FILES: [&str; 3] = ["config.json", RESET_FILES[0], RESET_FILES[1]];

pub(super) fn backup_state(options: &Options) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(&options.state_dir).map_err(|error| error.to_string())?;
    std::fs::create_dir_all(&options.output_dir).map_err(|error| error.to_string())?;
    let copied = copy_existing_files(&options.app_state_dir, &options.state_dir)?;
    if options.reset_trial {
        reset_trial_state(options)?;
    }
    Ok(copied)
}

pub(super) fn restore_state(options: &Options) -> Result<Vec<String>, String> {
    if !options.state_dir.is_dir() {
        return Err(format!(
            "manual QA state backup does not exist: {}",
            options.state_dir.display()
        ));
    }
    std::fs::create_dir_all(&options.app_state_dir).map_err(|error| error.to_string())?;
    let copied = copy_existing_files(&options.state_dir, &options.app_state_dir)?;
    if copied.is_empty() {
        return Err(format!(
            "manual QA state backup has no restorable files: {}",
            options.state_dir.display()
        ));
    }
    Ok(copied)
}

fn copy_existing_files(
    from: &std::path::Path,
    to: &std::path::Path,
) -> Result<Vec<String>, String> {
    let mut copied = Vec::new();
    for file in STATE_FILES {
        let source = from.join(file);
        if !source.exists() {
            continue;
        }
        std::fs::copy(&source, to.join(file)).map_err(|error| error.to_string())?;
        copied.push(file.to_string());
    }
    Ok(copied)
}

fn reset_trial_state(options: &Options) -> Result<(), String> {
    for file in RESET_FILES {
        let path = options.app_state_dir.join(file);
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}
