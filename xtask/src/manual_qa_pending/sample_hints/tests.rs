use super::{for_benchmark, for_manual, guidance_for};

fn text(csv: &std::path::Path) -> String {
    format!(
        "| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
        csv.display()
    )
}

fn csv() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("results.csv");
    std::fs::write(
        &csv,
        "backend,input,output\napple-native,/tmp/short.mov,/tmp/short.squashed.mp4\napple-native,/tmp/medium.mov,/tmp/medium.squashed.mp4\napple-native,/tmp/large.mov,/tmp/large.squashed.mp4\n",
    )
    .unwrap();
    (directory, csv)
}

#[test]
fn reports_hints_from_benchmark_sample_set_csv() {
    let (_directory, csv) = csv();
    let hints = for_manual(&text(&csv));
    assert_eq!(hints[0], "packaged-app small sample: /tmp/short.mov");
    assert_eq!(
        hints[4],
        "packaged-app not-smaller sample: /tmp/short.squashed.mp4"
    );
    assert_eq!(
        hints[5],
        "packaged-app sample aliases: /tmp/dropsquash-qa-open-panel/qa-small.mov, /tmp/dropsquash-qa-open-panel/qa-medium.mov, /tmp/dropsquash-qa-open-panel/qa-large.mov, /tmp/dropsquash-qa-open-panel/qa-not-smaller.mp4"
    );
}

#[test]
fn reports_benchmark_section_rules_from_sample_set_csv() {
    let (_directory, csv) = csv();
    let lines = for_benchmark(&text(&csv));

    assert_eq!(
        lines[0],
        "benchmark release-set inputs: short=/tmp/short.mov, medium=/tmp/medium.mov, large=/tmp/large.mov"
    );
    assert!(lines[1].contains("original local recordings"));
    assert!(lines[2].contains("shipping setting"));
    assert_eq!(
        lines[3],
        "benchmark release-set fallback: treat /tmp/dropsquash-qa-open-panel/qa-not-smaller.mp4 as a candidate only; if it still saves bytes under the current shipping profile and size, relink a different kept-original candidate before recording Larger output or rerunning the benchmark set"
    );
}

#[test]
fn reports_row_specific_guidance() {
    let (_directory, csv) = csv();
    let text = text(&csv);
    let queue = "sample: queue set (/tmp/short.mov, /tmp/medium.mov, /tmp/large.mov) aliases: /tmp/dropsquash-qa-open-panel/qa-small.mov, /tmp/dropsquash-qa-open-panel/qa-medium.mov, /tmp/dropsquash-qa-open-panel/qa-large.mov command: open /tmp/dropsquash-qa-open-panel";
    let medium = "sample: medium (/tmp/medium.mov) alias: /tmp/dropsquash-qa-open-panel/qa-medium.mov command: open /tmp/dropsquash-qa-open-panel";
    let not_smaller = "sample: not-smaller candidate (/tmp/short.squashed.mp4) alias: /tmp/dropsquash-qa-open-panel/qa-not-smaller.mp4 command: cargo run -p xtask -- manual-qa-open-chooser /tmp/dropsquash-qa-open-panel qa-not-smaller.mp4 and verify it still shows 'could not be made smaller', the row ends as Kept original, and original/trial/history stay unchanged before recording the result";
    let invalid = "sample: invalid alias /tmp/dropsquash-qa-open-panel/qa-invalid.mp4 after `cargo run -p xtask -- manual-qa-bad-input /tmp/dropsquash-qa-open-panel/qa-invalid.mp4`";
    assert_eq!(guidance_for("Batch summary", &text).as_deref(), Some(queue));
    assert_eq!(
        guidance_for("Trash source policy", &text).as_deref(),
        Some(medium)
    );
    assert_eq!(
        guidance_for("Larger output", &text).as_deref(),
        Some(not_smaller)
    );
    assert_eq!(
        guidance_for("Failed conversion", &text).as_deref(),
        Some(invalid)
    );
}
