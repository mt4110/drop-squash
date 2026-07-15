use std::path::Path;

pub(super) fn extra_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(artifact) = field_value(text, "App artifact") {
        lines.push(format!("packaged-app artifact: {artifact}"));
        lines.push(format!(
            "packaged-app open command: open -- '{}'",
            shell_single_quote(artifact)
        ));
        if artifact.ends_with(".dmg") {
            lines.push(
                "packaged-app stash installed app: cargo run -p xtask -- manual-qa-installed-app stash /tmp/dropsquash-manual-qa-installed-app".to_string(),
            );
            lines.push(
                "packaged-app restore installed app: cargo run -p xtask -- manual-qa-installed-app restore /tmp/dropsquash-manual-qa-installed-app".to_string(),
            );
        }
    }
    if let Some(path) = field_value(text, "Config path") {
        lines.push(format!("packaged-app config path: {path}"));
    }
    if let Some(path) = field_value(text, "Output folder") {
        lines.push(format!("packaged-app output folder: {path}"));
    }
    if let Some(path) = field_value(text, "History path") {
        lines.push(format!("packaged-app history path: {path}"));
    }
    if let Some(csv) = benchmark_csv(text) {
        lines.push(format!(
            "packaged-app sample link command: cargo run -p xtask -- manual-qa-link-samples '{}'",
            shell_single_quote(&csv)
        ));
        lines.push(format!(
            "packaged-app linked samples: {}, {}, {}",
            Path::new(&csv).with_file_name("qa-small.mov").display(),
            Path::new(&csv).with_file_name("qa-medium.mov").display(),
            Path::new(&csv).with_file_name("qa-large.mp4").display()
        ));
    }
    lines
}

fn benchmark_csv(text: &str) -> Option<String> {
    table_result(text, "Benchmark sample set")
        .and_then(crate::csv_evidence::existing_outside_repo_path)
        .map(|path| path.display().to_string())
}

fn table_result<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line.trim_matches('|').split('|').map(str::trim).collect::<Vec<_>>();
        match cells.as_slice() {
            [row, _, result] | [row, _, _, result] if *row == label => Some(*result),
            _ => None,
        }
    })
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line.trim_matches('|').split('|').map(str::trim).collect::<Vec<_>>();
        match cells.as_slice() {
            [found, value] if *found == label => Some(*value),
            _ => None,
        }
    })
}

fn shell_single_quote(text: &str) -> String {
    text.replace('\'', "'\\''")
}

#[cfg(test)]
mod tests {
    use super::extra_lines;

    #[test]
    fn reports_stash_and_link_commands_for_dmg_runs() {
        let directory = tempfile::tempdir().unwrap();
        let csv = directory.path().join("results.csv");
        std::fs::write(
            &csv,
            "backend,input\napple-native,/tmp/short.mov\napple-native,/tmp/medium.mov\napple-native,/tmp/large.mov\n",
        )
        .unwrap();
        let text = format!(
            "| App artifact | /tmp/DropSquash.dmg |\n| Output folder | /tmp/output |\n| Config path | /tmp/config.json |\n| History path | /tmp/history.jsonl |\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        );

        let lines = extra_lines(&text);

        assert!(lines.iter().any(|line| line.contains("manual-qa-installed-app stash")));
        assert!(lines.iter().any(|line| line.contains("manual-qa-link-samples")));
        assert!(lines.iter().any(|line| line.contains("qa-small.mov")));
    }
}
