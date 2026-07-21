use std::path::{Path, PathBuf};

const MAIN_MARKERS: [&str; 3] = [
    "single_instance::prepare()",
    "tauri_plugin_single_instance::init",
    "single_instance::focus_existing",
];
const HEIGHT_HOOK_MARKERS: [&str; 3] = [
    "useWindowHeight(shellRef, [",
    "state.isPro",
    "applicationsInstall.shouldShowNotice",
];
const HEIGHT_SYNC_MARKERS: [&str; 1] = ["setMinSize"];

pub(super) fn check_runtime_guards() -> Result<(), String> {
    require_markers(
        &workspace_path("apps/desktop/src-tauri/src/main.rs"),
        &MAIN_MARKERS,
        "desktop single-instance wiring",
    )?;
    require_markers(
        &workspace_path("apps/desktop/web/src/App.tsx"),
        &HEIGHT_HOOK_MARKERS,
        "desktop window-height hook wiring",
    )?;
    require_markers(
        &workspace_path("apps/desktop/web/src/hooks/useWindowHeight.ts"),
        &HEIGHT_SYNC_MARKERS,
        "desktop window-height resizing",
    )
}

fn workspace_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(path)
}

fn require_markers(path: &Path, markers: &[&str], label: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = markers
        .iter()
        .copied()
        .filter(|marker| !text.contains(marker))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{label} changed in {}; missing markers: {}",
        path.display(),
        missing.join(", ")
    ))
}

#[cfg(test)]
mod tests;
