use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-link-samples <results.csv> [link-dir]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (csv, dir) = parse_args(args)?;
    ensure_present(&csv)?;
    let rows = crate::benchmark_csv_check::read_rows(&csv)?;
    let links = create_links(&rows, &dir)?;
    for link in links {
        println!("{}", link.display());
    }
    for line in release_set_rules() {
        println!("{line}");
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf), String> {
    match args.as_slice() {
        [csv] => parent_dir(csv).map(|dir| (PathBuf::from(csv), dir)),
        [csv, dir] => Ok((PathBuf::from(csv), PathBuf::from(dir))),
        _ => Err(USAGE.to_string()),
    }
}

fn parent_dir(csv: &str) -> Result<PathBuf, String> {
    PathBuf::from(csv)
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "results.csv must have a parent directory".to_string())
}

fn ensure_present(csv: &Path) -> Result<(), String> {
    if csv.is_file() {
        return Ok(());
    }
    Err(format!(
        "benchmark CSV does not exist: {}. rerun `manual-qa-prepare`, then the printed `benchmark --release-set` and `benchmark-csv-check` commands, and use that fresh CSV path here",
        csv.display()
    ))
}

fn create_links(rows: &[Vec<String>], dir: &Path) -> Result<Vec<PathBuf>, String> {
    if rows.len() < 4 {
        return Err("benchmark CSV must include header and three sample rows".to_string());
    }
    std::fs::create_dir_all(dir).map_err(|error| format!("failed to create link dir: {error}"))?;
    let samples = rows.iter().skip(1).take(3).collect::<Vec<_>>();
    let [small, medium, large] = samples.as_slice() else {
        return Err("benchmark CSV must include header and three sample rows".to_string());
    };
    Ok(vec![
        link_input(dir, "small", small)?,
        link_input(dir, "medium", medium)?,
        link_input(dir, "large", large)?,
        link_output(dir, "not-smaller", small)?,
    ])
}

fn link_input(dir: &Path, label: &str, row: &[String]) -> Result<PathBuf, String> {
    let source = PathBuf::from(
        row.get(1)
            .ok_or_else(|| format!("benchmark CSV row is missing input column: {label}"))?,
    );
    link(dir, label, &source)
}

fn link_output(dir: &Path, label: &str, row: &[String]) -> Result<PathBuf, String> {
    let source = PathBuf::from(
        row.get(2)
            .ok_or_else(|| format!("benchmark CSV row is missing output column: {label}"))?,
    );
    link(dir, label, &source)
}

fn link(dir: &Path, label: &str, source: &Path) -> Result<PathBuf, String> {
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("sample has no file extension: {}", source.display()))?;
    let link = dir.join(format!("qa-{label}.{extension}"));
    if link_present(&link) {
        std::fs::remove_file(&link)
            .map_err(|error| format!("failed to replace {}: {error}", link.display()))?;
    }
    std::os::unix::fs::symlink(source, &link)
        .map_err(|error| format!("failed to link {}: {error}", link.display()))?;
    Ok(link)
}

fn link_present(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}

fn release_set_rules() -> [&'static str; 4] {
    [
        "release-set rule: use original local recordings, not prior .squashed outputs, for short/medium/large benchmark inputs",
        "release-set rule: keep profile and size aligned with the shipping setting under test before recording benchmark evidence",
        "release-set rule: avoid already tiny, nearly static, or otherwise re-encoded delivery clips when choosing benchmark replacements",
        "release-set rule: qa-not-smaller is only a candidate alias; verify it still produces a kept-original not-smaller result under the current shipping profile and size before recording the packaged-app Larger output row",
    ]
}
