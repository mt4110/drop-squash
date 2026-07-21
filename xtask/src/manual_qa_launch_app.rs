mod args;
mod paths;

use args::parse_args;
use paths::{home_dir, resolve_app, state_dir};

const PANEL_DIR: &str = "/tmp/dropsquash-qa-open-panel";
const INSTALLED_APP: &str = "/Applications/DropSquash.app";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-launch-app [--event-log] [--mount-dmg] [--open-panel] [--open-file <path>] [--settle-seconds <seconds>] [--license-api-base-url <url>] <artifact-path> <config-path>";
pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (
        event_log,
        mount_dmg,
        open_panel,
        open_file,
        settle_seconds,
        license_api_base_url,
        artifact,
        config,
    ) = parse_args(args)?;
    ensure_dmg_isolated(&artifact)?;
    let app = resolve_app(&artifact, mount_dmg)?;
    let home = home_dir(&config)
        .ok_or_else(|| format!("failed to derive DropSquash home from {config}"))?;
    let state =
        state_dir(&config).ok_or_else(|| format!("failed to derive state dir from {config}"))?;
    let mut command = std::process::Command::new("open");
    command.args(["-n", "-a", &app]);
    command.env("DROP_SQUASH_HOME", home);
    command.env("DROP_SQUASH_APP_STATE_DIR", state);
    if event_log {
        command.env(
            "DROP_SQUASH_MANUAL_QA_EVENT_LOG",
            "/tmp/dsq-drag-events.jsonl",
        );
    }
    if let Some(url) = license_api_base_url.as_deref() {
        command.env("DROP_SQUASH_LICENSE_API_BASE_URL", url);
    }
    command
        .spawn()
        .map_err(|error| format!("failed to launch app: {error}"))?;
    if open_panel {
        open_panel_dir()?;
    }
    if let Some(path) = open_file.as_deref() {
        open_file_in_app(&app, path, settle_seconds)?;
    }
    println!("manual QA app launched");
    Ok(())
}

fn open_panel_dir() -> Result<(), String> {
    std::process::Command::new("open")
        .arg(PANEL_DIR)
        .status()
        .map_err(|error| format!("failed to open panel dir: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "open panel dir command failed".to_string())
}
fn open_file_in_app(app: &str, path: &str, settle_seconds: u64) -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_secs(settle_seconds));
    std::process::Command::new("open")
        .args(["-a", app, path])
        .status()
        .map_err(|error| format!("failed to open sample file: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "open sample file command failed".to_string())
}
fn ensure_dmg_isolated(artifact: &str) -> Result<(), String> {
    if artifact.ends_with(".dmg") && std::path::Path::new(INSTALLED_APP).exists() {
        return Err("installed app exists at /Applications/DropSquash.app; run `cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app` before DMG QA".to_string());
    }
    Ok(())
}
#[cfg(test)]
mod tests;
