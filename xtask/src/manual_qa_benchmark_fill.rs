use std::path::{Path, PathBuf};

mod markdown;

#[cfg(test)]
mod tests;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-fill-benchmark <manual-qa.md> <results.csv>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (manual, csv) = parse_args(args)?;
    crate::benchmark_csv_check::validate_path(&csv)?;
    let text = std::fs::read_to_string(&manual)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let filled = fill_rows(&text, &summary(&csv, &text)?)?;
    std::fs::write(&manual, filled)
        .map_err(|error| format!("failed to write manual QA file: {error}"))?;
    println!("filled benchmark rows: {}", manual.display());
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf), String> {
    match args.as_slice() {
        [manual, csv] if manual != "--help" && manual != "-h" => {
            Ok((PathBuf::from(manual), PathBuf::from(csv)))
        }
        _ => Err(USAGE.to_string()),
    }
}

fn summary(csv: &Path, manual: &str) -> Result<(String, String), String> {
    let rows = crate::benchmark_csv_check::read_rows(csv)?;
    let [_, a, b, c] = rows.as_slice() else {
        return Err("benchmark CSV must contain exactly three sample rows".to_string());
    };
    let backend = cell(a, 0)?;
    if [a, b, c]
        .iter()
        .any(|row| cell(row, 0).ok() != Some(backend))
    {
        return Err("benchmark CSV must use one backend for all samples".to_string());
    }
    let machine = field(manual, "Machine")?;
    let macos = field(manual, "macOS version")?;
    Ok((
        format!(
            "benchmark CSV recorded for three samples with smaller outputs outside repo at {}",
            csv.display()
        ),
        format!(
            "three short, medium, and large samples produced smaller outputs with backend {backend}, saved percent, duration, and speed ratio on {machine} {macos} with CSV saved outside repo at {}: {}; {}; {}",
            csv.display(),
            metric(a)?,
            metric(b)?,
            metric(c)?,
        ),
    ))
}

fn fill_rows(text: &str, summary: &(String, String)) -> Result<String, String> {
    let mut command = false;
    let mut sample = false;
    let lines = text
        .lines()
        .map(|line| match markdown::label(line) {
            Some("`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`") => {
                command = true;
                markdown::replace(line, &summary.0)
            }
            Some("Benchmark sample set") => {
                sample = true;
                markdown::replace(line, &summary.1)
            }
            _ => line.to_string(),
        })
        .collect::<Vec<_>>();
    (command && sample)
        .then(|| lines.join("\n"))
        .ok_or_else(|| "manual QA file is missing benchmark rows".to_string())
}

fn field<'a>(manual: &'a str, name: &str) -> Result<&'a str, String> {
    markdown::field(manual, name).ok_or_else(|| format!("manual QA field is missing: {name}"))
}

fn metric(row: &[String]) -> Result<String, String> {
    Ok(format!(
        "{} {:.3}s {:.1}% saved {:.3} MiB/s {:.3}x speed ratio",
        name(cell(row, 1)?),
        number(row, 5)?,
        number(row, 8)?,
        number(row, 9)?,
        number(row, 10)?,
    ))
}

fn cell(row: &[String], index: usize) -> Result<&str, String> {
    row.get(index)
        .map(|value| value.as_str())
        .ok_or_else(|| format!("benchmark CSV is missing column {index}"))
}

fn number(row: &[String], index: usize) -> Result<f64, String> {
    cell(row, index)?
        .parse::<f64>()
        .map_err(|_| format!("benchmark CSV has invalid number in column {index}"))
}

fn name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}
