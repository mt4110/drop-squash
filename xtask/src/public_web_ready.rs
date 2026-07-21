use std::path::Path;
use std::process::Command;

#[cfg(test)]
mod tests;

const SITE_DIR: &str = "apps/site";
const USAGE: &str = "usage: cargo run -p xtask -- public-web-ready";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(USAGE.to_string());
    }
    crate::website_check::check_default_root()?;
    verify_site(Path::new(SITE_DIR))?;
    for line in success_lines() {
        println!("{line}");
    }
    for line in crate::public_web_rerun::next_lines() {
        println!("{line}");
    }
    Ok(())
}

fn verify_site(path: &Path) -> Result<(), String> {
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
    let details = if !stderr.is_empty() { stderr } else { stdout };
    Err(format!(
        "npm --prefix {} run verify:site failed: {}\n{}",
        path.display(),
        output.status,
        if details.is_empty() {
            "no output captured"
        } else {
            &details
        }
    ))
}

fn success_lines() -> [String; 3] {
    let today = crate::current_date::display();
    [
        "public web local gate passed: cargo run -p xtask -- website-check".to_string(),
        format!("public web deployable site passed: npm --prefix {SITE_DIR} run verify:site"),
        format!(
            "public web ready: local website content and deployable site packaging passed on {today}"
        ),
    ]
}
