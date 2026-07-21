pub(super) fn resolve_app(artifact: &str, mount_dmg: bool) -> Result<String, String> {
    if mount_dmg && artifact.ends_with(".dmg") {
        return mounted_app_bundle(artifact);
    }
    app_bundle(artifact).ok_or_else(|| format!("failed to resolve app bundle from {artifact}"))
}

pub(super) fn home_dir(config_path: &str) -> Option<String> {
    std::path::Path::new(config_path)
        .parent()?
        .parent()?
        .parent()?
        .parent()
        .map(display)
}

pub(super) fn state_dir(config_path: &str) -> Option<String> {
    std::path::Path::new(config_path).parent().map(display)
}

pub(super) fn app_bundle(artifact: &str) -> Option<String> {
    if artifact.ends_with(".app") {
        return Some(artifact.to_string());
    }
    let dmg = std::path::Path::new(artifact);
    dmg.parent()?
        .parent()?
        .join("macos")
        .join("DropSquash.app")
        .exists()
        .then(|| {
            display(
                dmg.parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("macos")
                    .join("DropSquash.app"),
            )
        })
}

pub(super) fn mounted_app_bundle(artifact: &str) -> Result<String, String> {
    let output = std::process::Command::new("hdiutil")
        .args(["attach", "-nobrowse", "-readonly", artifact])
        .output()
        .map_err(|error| format!("failed to mount dmg: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "hdiutil attach failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    let mount = stdout
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find(|part| part.starts_with("/Volumes/"))
        })
        .ok_or_else(|| "failed to find mounted dmg volume".to_string())?;
    Ok(format!("{mount}/DropSquash.app"))
}

pub(super) fn display(path: impl AsRef<std::path::Path>) -> String {
    path.as_ref().display().to_string()
}
