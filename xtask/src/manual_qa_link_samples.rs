use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-link-samples <results.csv> [link-dir]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (csv, dir) = parse_args(args)?;
    let rows = crate::benchmark_csv_check::read_rows(&csv)?;
    let links = create_links(&rows, &dir)?;
    for link in links {
        println!("{}", link.display());
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

fn create_links(rows: &[Vec<String>], dir: &Path) -> Result<Vec<PathBuf>, String> {
    if rows.len() < 4 {
        return Err("benchmark CSV must include header and three sample rows".to_string());
    }
    std::fs::create_dir_all(dir).map_err(|error| format!("failed to create link dir: {error}"))?;
    ["small", "medium", "large"]
        .into_iter()
        .zip(rows.iter().skip(1).take(3))
        .map(|(label, row)| link_row(dir, label, row))
        .collect()
}

fn link_row(dir: &Path, label: &str, row: &[String]) -> Result<PathBuf, String> {
    let source = PathBuf::from(
        row.get(1)
            .ok_or_else(|| format!("benchmark CSV row is missing input column: {label}"))?,
    );
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("sample has no file extension: {}", source.display()))?;
    let link = dir.join(format!("qa-{label}.{extension}"));
    if link.exists() {
        std::fs::remove_file(&link)
            .map_err(|error| format!("failed to replace {}: {error}", link.display()))?;
    }
    std::os::unix::fs::symlink(&source, &link)
        .map_err(|error| format!("failed to link {}: {error}", link.display()))?;
    Ok(link)
}
