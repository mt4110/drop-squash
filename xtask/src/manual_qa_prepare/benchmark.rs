use std::path::PathBuf;

use super::options::Options;

pub(super) fn print_plan(options: &Options) {
    let csv = csv_output(options);
    println!("manual QA Benchmark command:");
    println!(
        "cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir {} --csv-output {}",
        options.output_dir.display(),
        csv.display()
    );
    println!("manual QA Benchmark CSV check:");
    println!(
        "cargo run -p xtask -- benchmark-csv-check {}",
        csv.display()
    );
}

fn csv_output(options: &Options) -> PathBuf {
    options
        .output_dir
        .parent()
        .map(|parent| parent.join("dropsquash-benchmark-results.csv"))
        .unwrap_or_else(|| PathBuf::from("/tmp/dropsquash-benchmark-results.csv"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::csv_output;
    use crate::manual_qa_prepare::options::Options;

    #[test]
    fn places_csv_next_to_output_parent() {
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

        assert_eq!(
            csv_output(&options),
            PathBuf::from("/tmp/dropsquash-benchmark-results.csv")
        );
    }
}
