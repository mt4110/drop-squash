use std::path::{Path, PathBuf};

use super::{markdown::Field, options::Options};

pub(super) fn print_plan(options: &Options, csv: &Path) {
    println!("manual QA Benchmark command:");
    println!("{}", benchmark_command(options, csv));
    println!("manual QA Benchmark CSV check:");
    println!("{}", csv_check_command(csv));
    println!("manual QA Benchmark observation rows:");
    for row in rows() {
        println!("{row}");
    }
}

pub(super) fn csv_output(fields: &[Field], output_dir: &Path) -> PathBuf {
    let suffix = fields
        .iter()
        .find(|(label, _)| *label == "App build")
        .and_then(|(_, value)| value.split_whitespace().last())
        .filter(|value| value.len() >= 7)
        .unwrap_or("current");
    output_dir.join(format!("benchmark-results-{suffix}.csv"))
}

pub(super) fn benchmark_command(options: &Options, csv: &Path) -> String {
    format!(
        "cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir {} --csv-output {}",
        options.output_dir.display(),
        csv.display()
    )
}

pub(super) fn csv_check_command(csv: &Path) -> String {
    format!("cargo run -p xtask -- benchmark-csv-check {}", csv.display())
}

pub(super) fn rows() -> Vec<String> {
    vec![
        format!(
            "| {} | Existing absolute `.csv` path recorded outside repo for three local samples; outputs are smaller |  |",
            command_label()
        ),
        "| Benchmark sample set | Three short, medium, and large private local recordings produce smaller outputs and are recorded with backend, saved percent, duration, speed ratio, existing absolute CSV path outside repo, machine, and OS context |  |".to_string(),
        "| Benchmark regression threshold | Throughput does not regress by more than 20% on two or more samples against the same-machine release candidate baseline without a documented reason |  |".to_string(),
    ]
}

fn command_label() -> &'static str {
    "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{csv_output, rows};
    use crate::manual_qa_prepare::options::Options;

    #[test]
    fn places_csv_inside_prepared_output_directory() {
        let options = Options {
            app_artifact: None,
            app_state_dir: PathBuf::from("/tmp/app-state"),
            input_sample_set: None,
            markdown_output: None,
            output_dir: PathBuf::from("/tmp/dropsquash-manual-qa-output"),
            reset_trial: false,
            restore_state: false,
            state_dir: PathBuf::from("/tmp/state"),
        };
        let fields = vec![("App build", "DropSquash 0.1.0 git abc1234".to_string())];

        assert_eq!(
            csv_output(&fields, &options.output_dir),
            PathBuf::from("/tmp/dropsquash-manual-qa-output/benchmark-results-abc1234.csv")
        );
    }

    #[test]
    fn generated_rows_match_required_manual_qa_checks() {
        let rows = rows().join("\n");

        for check in [
            "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`",
            "Benchmark sample set",
            "Benchmark regression threshold",
        ] {
            assert!(crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&check));
            assert!(rows.contains(check));
        }
    }

    #[test]
    fn generated_rows_exist_in_manual_qa_template() {
        let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();

        for row in rows() {
            let check = row.split('|').nth(1).unwrap().trim();
            assert!(template.contains(check));
        }
    }
}
