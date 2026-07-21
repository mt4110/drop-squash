use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-installed-app <status|stash|restore> <backup-dir> [app-path]";
const DEFAULT_APP: &str = "/Applications/DropSquash.app";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (mode, backup, app) = parse_args(args)?;
    let moved = match mode.as_str() {
        "status" => return status(&app, &backup),
        "stash" => stash(&app, &backup)?,
        "restore" => restore(&app, &backup)?,
        _ => return Err(USAGE.to_string()),
    };
    println!("{}", moved.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, PathBuf, PathBuf), String> {
    match args.as_slice() {
        [mode, backup] => Ok((
            mode.clone(),
            PathBuf::from(backup),
            PathBuf::from(DEFAULT_APP),
        )),
        [mode, backup, app] => Ok((mode.clone(), PathBuf::from(backup), PathBuf::from(app))),
        _ => Err(USAGE.to_string()),
    }
}

fn stash(app: &Path, backup_dir: &Path) -> Result<PathBuf, String> {
    if !app.exists() {
        return Err(format!("installed app does not exist: {}", app.display()));
    }
    std::fs::create_dir_all(backup_dir).map_err(|error| error.to_string())?;
    let backup = backup_dir.join(file_name(app)?);
    if backup.exists() {
        return Err(format!("backup app already exists: {}", backup.display()));
    }
    std::fs::rename(app, &backup).map_err(|error| error.to_string())?;
    Ok(backup)
}

fn restore(app: &Path, backup_dir: &Path) -> Result<PathBuf, String> {
    if app.exists() {
        return Err(format!("installed app already exists: {}", app.display()));
    }
    let backup = backup_dir.join(file_name(app)?);
    if !backup.exists() {
        return Err(format!("backup app does not exist: {}", backup.display()));
    }
    if let Some(parent) = app.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    std::fs::rename(&backup, app).map_err(|error| error.to_string())?;
    Ok(app.to_path_buf())
}

fn status(app: &Path, backup_dir: &Path) -> Result<(), String> {
    let backup = backup_dir.join(file_name(app)?);
    let backup_dir = backup_dir.display();
    println!("installed app: {}", exists_label(app));
    println!("backup app: {}", exists_label(&backup));
    println!("mounted dmg qa: {}", mounted_dmg_label(app, &backup));
    println!("app path: {}", app.display());
    println!("backup path: {}", backup.display());
    if app.exists() {
        println!("next command: cargo run -p xtask -- manual-qa-installed-app stash {backup_dir}");
    } else {
        println!("next command: mounted DMG QA can start");
        if backup.exists() {
            println!("restore command: cargo run -p xtask -- manual-qa-installed-app restore {backup_dir}");
        }
    }
    Ok(())
}

fn exists_label(path: &Path) -> &'static str {
    if path.exists() {
        "present"
    } else {
        "missing"
    }
}

fn mounted_dmg_label(app: &Path, backup: &Path) -> &'static str {
    match (app.exists(), backup.exists()) {
        (true, true) => "restore-before-stash",
        (true, false) => "stash-required",
        (false, _) => "ready",
    }
}

fn file_name(path: &Path) -> Result<&std::ffi::OsStr, String> {
    path.file_name()
        .ok_or_else(|| format!("app path has no file name: {}", path.display()))
}
