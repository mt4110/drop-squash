use std::path::Path;
use std::process::Command;

pub(super) fn check(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!("missing site directory: {}", path.display()));
    }
    if !path.join("package.json").is_file() {
        return Err(format!("missing package.json: {}", path.display()));
    }
    let output = Command::new("npm")
        .arg("--prefix")
        .arg(path)
        .arg("run")
        .arg("verify:site")
        .output()
        .map_err(|error| format!("failed to run npm verify:site: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let details = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        "no output captured".to_string()
    };
    Err(format!(
        "npm --prefix {} run verify:site failed: {}\n{}",
        path.display(),
        output.status,
        details
    ))
}
