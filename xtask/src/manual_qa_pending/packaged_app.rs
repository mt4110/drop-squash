mod launch;
mod samples;
use super::fields::{field_value, table_result};
use std::path::Path;
const PANEL_DIR: &str = "/tmp/dropsquash-qa-open-panel";
const FRESH_APP: &str = "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app";
#[cfg(test)]
mod tests;

pub(super) fn extra_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("packaged-app fresh build command: CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build".to_string());
    lines.push(format!("packaged-app fresh app artifact: {FRESH_APP}"));
    if let Some(artifact) = field_value(text, "App artifact") {
        lines.push(format!("packaged-app artifact: {artifact}"));
        lines.push(format!(
            "packaged-app open command: open -- '{}'",
            shell_single_quote(artifact)
        ));
        if artifact.ends_with(".dmg") {
            lines.push("packaged-app installed app status: cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app".to_string());
            lines.push("packaged-app stash installed app: cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app".to_string());
            lines.push("packaged-app restore installed app: cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app".to_string());
            if Path::new("/Applications/DropSquash.app").exists() {
                lines.push("packaged-app installed app note: /Applications/DropSquash.app exists, so stash it before mounted DMG QA to avoid helper refusal".to_string());
            }
            lines.push("packaged-app mounted dmg screenshot command: cargo run -p xtask -- manual-qa-window-capture /tmp/dropsquash-mounted.png".to_string());
            lines.push("packaged-app mounted dmg window probe command: cargo run -p xtask -- manual-qa-window-probe".to_string());
            lines.push("packaged-app mounted dmg pid count command: pgrep -f '/Volumes/DropSquash/DropSquash.app/Contents/MacOS/dropsquash-desktop' | wc -l".to_string());
            lines.push("packaged-app mounted dmg pid list command: pgrep -f '/Volumes/DropSquash/DropSquash.app/Contents/MacOS/dropsquash-desktop' | sort".to_string());
        }
    }
    if let Some(path) = field_value(text, "Config path") {
        lines.push(format!("packaged-app config path: {path}"));
        lines.extend(launch::lines(field_value(text, "App artifact"), Some(path)));
        if let Some(artifact) = field_value(text, "App artifact") {
            lines.push(format!("packaged-app launch helper command: cargo run -p xtask -- manual-qa-launch-app '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
            lines.push(format!("packaged-app fresh app launch helper command: cargo run -p xtask -- manual-qa-launch-app '{}' '{}'", FRESH_APP, shell_single_quote(path)));
            lines.push(format!("packaged-app launch + panel helper command: cargo run -p xtask -- manual-qa-launch-app --open-panel '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
            lines.push(format!("packaged-app drag event log helper command: cargo run -p xtask -- manual-qa-launch-app --event-log --open-panel '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
            if artifact.ends_with(".dmg") {
                lines.push(format!("packaged-app mounted dmg helper command: cargo run -p xtask -- manual-qa-launch-app --mount-dmg '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
                lines.push(format!("packaged-app mounted dmg + panel helper command: cargo run -p xtask -- manual-qa-launch-app --mount-dmg --open-panel '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
                lines.push(format!("packaged-app mounted dmg drag event helper command: cargo run -p xtask -- manual-qa-launch-app --mount-dmg --event-log --open-panel '{}' '{}'", shell_single_quote(artifact), shell_single_quote(path)));
                lines.push("packaged-app mounted dmg screenshot note: after the mounted app window appears, run the screenshot helper above to capture the disk-image notice before pressing Move or OK".to_string());
                lines.push("packaged-app mounted dmg pid note: record pid count before relaunch, relaunch the mounted app once, then confirm the pid count and pid list stay unchanged".to_string());
                lines.push("packaged-app mounted dmg relaunch note: while the mounted-app window is still visible, relaunch DropSquash and confirm the existing window takes focus, the AX window count stays at 1, and the mounted app pid count does not increase".to_string());
            }
        }
    }
    if let Some(path) = field_value(text, "Output folder") {
        lines.push(format!("packaged-app output folder: {path}"));
        if text.contains("| Drag-and-drop conversion |") {
            lines.push(format!(
                "packaged-app drag output inspect command: ls -lt '{}' | head",
                shell_single_quote(path)
            ));
        }
    }
    if let Some(path) = field_value(text, "History path") {
        lines.push(format!("packaged-app history path: {path}"));
        if text.contains("| Drag-and-drop conversion |") {
            lines.push(format!(
                "packaged-app drag history inspect command: tail -n 5 '{}'",
                shell_single_quote(path)
            ));
            if let Some(output_dir) = field_value(text, "Output folder") {
                lines.push(format!("packaged-app drag report helper command: cargo run -p xtask -- manual-qa-drag-report '{}' '{}'", shell_single_quote(output_dir), shell_single_quote(path)));
            }
        }
    }
    if text.contains("| Drag-and-drop conversion |") {
        lines.push("packaged-app drag note: synthetic drag is diagnostic only; record Drag-and-drop conversion from a real Finder drag into the packaged app".to_string());
        lines.push(crate::manual_qa_observation::packaged_visibility_note(
            "packaged-app visibility note",
        ));
        lines.push(format!("packaged-app panel sample dir: {PANEL_DIR}"));
        lines.push(format!(
            "packaged-app panel sample open command: open {PANEL_DIR}"
        ));
        lines.push(format!("packaged-app drag diagnostic command: cargo run -p xtask -- manual-qa-drag-drop {PANEL_DIR} qa-small.mov"));
        lines.push("packaged-app drag event log path: /tmp/dsq-drag-events.jsonl".to_string());
        lines.push("packaged-app drag event log inspect command: if [ -f /tmp/dsq-drag-events.jsonl ]; then cat /tmp/dsq-drag-events.jsonl; else echo '__MISSING__'; fi".to_string());
        lines.push("packaged-app drag event log launch note: for local diagnostics, start the packaged app with DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-drag-events.jsonl before doing the real Finder drag".to_string());
        lines.push("packaged-app drag result template: Dragging <sample>.mov from Finder into the packaged app saved <name>.squashed.mp4, the output was smaller than the original, and the original remained in place".to_string());
    }
    lines.extend(samples::lines(text, PANEL_DIR, FRESH_APP));
    lines
}

pub(super) fn benchmark_csv(text: &str) -> Option<String> {
    table_result(text, "Benchmark sample set")
        .and_then(crate::csv_evidence::existing_outside_repo_path)
        .map(|path| path.display().to_string())
}

pub(super) fn shell_single_quote(text: &str) -> String {
    text.replace('\'', "'\\''")
}
