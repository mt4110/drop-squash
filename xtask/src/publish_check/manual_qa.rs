use std::path::Path;

pub(super) fn require_public_dmg(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA {}: {error}", path.display()))?;
    let Some(artifact) = app_artifact(&text) else {
        return Err("manual QA App artifact must be present before publish".to_string());
    };
    require_dmg_artifact(Path::new(artifact))
}

fn require_dmg_artifact(path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("manual QA App artifact must be an absolute path before publish".to_string());
    }
    if !path.is_file() {
        return Err("manual QA App artifact must exist before publish".to_string());
    }
    if path.file_name().and_then(|name| name.to_str()) != Some("DropSquash.dmg") {
        return Err(
            "manual QA App artifact must be the public DropSquash.dmg before publish".to_string(),
        );
    }
    crate::dmg::read(path, "publish manual QA App artifact")?;
    Ok(())
}

pub(super) fn require_current_head(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read manual QA {}: {error}", path.display()))?;
    let Some(build) = field_value(&text, "App build") else {
        return Err("manual QA App build must be present before publish".to_string());
    };
    if crate::git_head_match::contains_current_short_head_after_git(build)? {
        return Ok(());
    }
    Err("manual QA App build must match current HEAD before publish".to_string())
}

fn app_artifact(text: &str) -> Option<&str> {
    field_value(text, "App artifact")
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        (cells.len() == 2 && cells.first() == Some(&label)).then_some(cells[1])
    })
}

#[cfg(test)]
fn git_head() -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

#[cfg(test)]
mod tests;
