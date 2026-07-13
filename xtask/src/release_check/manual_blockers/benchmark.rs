const COMMAND: &str =
    "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`";
const SAMPLE_SET: &str = "Benchmark sample set";

pub(super) fn lacks_matching_csv(manual: &str) -> bool {
    let Some(command_csv) = csv_for(manual, COMMAND) else {
        return false;
    };
    let Some(sample_csv) = csv_for(manual, SAMPLE_SET) else {
        return false;
    };
    command_csv != sample_csv
}

fn csv_for(manual: &str, check: &str) -> Option<std::path::PathBuf> {
    manual
        .lines()
        .find(|line| matches_check(line, check))
        .and_then(|line| result_cell(line))
        .and_then(crate::csv_evidence::canonical_existing_path)
}

fn matches_check(line: &str, check: &str) -> bool {
    line.starts_with('|') && line.trim_matches('|').split('|').next().map(str::trim) == Some(check)
}

fn result_cell(line: &str) -> Option<&str> {
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    matches!(cells.len(), 3..=4).then(|| cells[cells.len() - 1])
}
