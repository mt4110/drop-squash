use crate::manual_qa_pending::fields::field_value;
use std::path::Path;

use super::{benchmark_csv, shell_single_quote};

pub(super) fn lines(text: &str, panel_dir: &str, fresh_app: &str) -> Vec<String> {
    let Some(csv) = benchmark_csv(text) else {
        return Vec::new();
    };
    let aliases = sample_aliases(&csv).unwrap_or_else(default_aliases);
    let artifact = shell_single_quote(field_value(text, "App artifact").unwrap_or(""));
    let config = shell_single_quote(field_value(text, "Config path").unwrap_or(""));
    let mut lines = vec![
        format!("packaged-app sample link command: cargo run -p xtask -- manual-qa-link-samples '{}'", shell_single_quote(&csv)),
        format!("packaged-app panel sample link command: cargo run -p xtask -- manual-qa-link-samples '{}' {panel_dir}", shell_single_quote(&csv)),
        format!("packaged-app chooser command: cargo run -p xtask -- manual-qa-open-chooser {panel_dir}"),
        format!("packaged-app drag sample command: cargo run -p xtask -- manual-qa-drag-drop {panel_dir} {}", aliases[0]),
        format!("packaged-app chooser small sample command: cargo run -p xtask -- manual-qa-open-chooser {panel_dir} {}", aliases[0]),
        format!("packaged-app chooser medium sample command: cargo run -p xtask -- manual-qa-open-chooser {panel_dir} {}", aliases[1]),
        format!("packaged-app chooser large sample command: cargo run -p xtask -- manual-qa-open-chooser {panel_dir} {}", aliases[2]),
        format!("packaged-app chooser not smaller sample command: cargo run -p xtask -- manual-qa-open-chooser {panel_dir} qa-not-smaller.mp4"),
        format!("packaged-app open-file sample command: cargo run -p xtask -- manual-qa-launch-app --open-file {panel_dir}/{} '{artifact}' '{config}'", aliases[0]),
        format!("packaged-app open-file not-smaller command: cargo run -p xtask -- manual-qa-launch-app --settle-seconds 9 --open-file {panel_dir}/qa-not-smaller.mp4 '{artifact}' '{config}'"),
        format!("packaged-app fresh open-file not-smaller command: cargo run -p xtask -- manual-qa-launch-app --settle-seconds 9 --open-file {panel_dir}/qa-not-smaller.mp4 '{fresh_app}' '{config}'"),
        "packaged-app chooser not smaller sample note: qa-not-smaller.mp4 is a candidate alias; if it saves bytes under the current shipping profile and size, relink a different kept-original candidate before recording Larger output".to_string(),
        format!("packaged-app invalid sample command: cargo run -p xtask -- manual-qa-bad-input {panel_dir}/qa-invalid.mp4"),
        format!("packaged-app invalid sample: {panel_dir}/qa-invalid.mp4"),
        format!("packaged-app linked samples: {}, {}, {}, {}", Path::new(&csv).with_file_name(&aliases[0]).display(), Path::new(&csv).with_file_name(&aliases[1]).display(), Path::new(&csv).with_file_name(&aliases[2]).display(), Path::new(&csv).with_file_name("qa-not-smaller.mp4").display()),
        format!("packaged-app panel linked samples: {panel_dir}/{}, {panel_dir}/{}, {panel_dir}/{}, {panel_dir}/qa-not-smaller.mp4", aliases[0], aliases[1], aliases[2]),
    ];
    if let (Some(app), Some(cfg)) = (
        field_value(text, "App artifact"),
        field_value(text, "Config path"),
    ) {
        lines.push(format!("packaged-app quickstart 1: cargo run -p xtask -- manual-qa-link-samples '{}' {panel_dir}", shell_single_quote(&csv)));
        if app.ends_with(".dmg") {
            lines.push("packaged-app quickstart 2: cargo run -p xtask -- manual-qa-installed-app status /tmp/dropsquash-manual-qa-installed-app".to_string());
            lines.push("packaged-app quickstart 3: cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app".to_string());
            lines.push(format!("packaged-app quickstart 4: cargo run -p xtask -- manual-qa-launch-app --mount-dmg --open-panel '{}' '{}'", shell_single_quote(app), shell_single_quote(cfg)));
            lines.push(format!("packaged-app quickstart 5: cargo run -p xtask -- manual-qa-launch-app --mount-dmg --open-file {panel_dir}/{} '{}' '{}'", aliases[0], shell_single_quote(app), shell_single_quote(cfg)));
            lines.push(format!("packaged-app quickstart 6: for real evidence, drag {} from Finder into the mounted DropSquash app; use the synthetic drag command below only for diagnostics", aliases[0]));
        } else {
            lines.push(format!("packaged-app quickstart 2: cargo run -p xtask -- manual-qa-launch-app --open-panel '{}' '{}'", shell_single_quote(app), shell_single_quote(cfg)));
            lines.push(format!("packaged-app quickstart 3: cargo run -p xtask -- manual-qa-launch-app --open-file {panel_dir}/{} '{}' '{}'", aliases[0], shell_single_quote(app), shell_single_quote(cfg)));
            lines.push(format!("packaged-app quickstart 4: for real evidence, drag {} from Finder into the packaged app; use the synthetic drag command below only for diagnostics", aliases[0]));
        }
    }
    lines
}

fn sample_aliases(csv: &str) -> Option<[String; 3]> {
    let rows = crate::benchmark_csv_check::read_rows(Path::new(csv)).ok()?;
    Some([
        alias_name("small", rows.get(1)?)?,
        alias_name("medium", rows.get(2)?)?,
        alias_name("large", rows.get(3)?)?,
    ])
}

fn alias_name(label: &str, row: &[String]) -> Option<String> {
    Some(format!(
        "qa-{label}.{}",
        Path::new(row.get(1)?).extension()?.to_str()?
    ))
}

fn default_aliases() -> [String; 3] {
    ["qa-small.mov", "qa-medium.mov", "qa-large.mp4"].map(str::to_string)
}
